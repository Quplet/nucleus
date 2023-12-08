use bevy::{prelude::*, input::mouse::MouseWheel};

use crate::{GameState, util::{clamp, clamp_vec2_by_length}, level_manager::LevelStats, collision::{Collider, is_colliding, is_cords_in_collider}, atom::Atom, neutron::{NEUTRON_SIZE, Neutron, PlacementMarker, spawn_neutron_with_marker}};

const MAX_ZOOM_OUT: f32 = 10.;
const MAX_ZOOM_IN: f32 = 0.1;

#[derive(Resource)]
pub struct CameraOptions {
    zoom_speed: f32,
    movement_speed: f32
}

impl Default for CameraOptions {
    fn default() -> Self {
        CameraOptions { zoom_speed: 1./15., movement_speed: 100. }
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum PlacementState {
    #[default]
    NEUTRON,
    VELOCITY
}

pub fn camera_zoom(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    camera_options: Res<CameraOptions>,
    mut cameras: Query<&mut Transform, With<Camera>>
) {
    let mut camera_transform = cameras.single_mut();

    for mouse_wheel_event in mouse_wheel_events.read() {
        let zoom_value = -mouse_wheel_event.y * camera_options.zoom_speed;
        
        camera_transform.scale.x = clamp(camera_transform.scale.x + zoom_value, MAX_ZOOM_IN, MAX_ZOOM_OUT);
        camera_transform.scale.y = clamp(camera_transform.scale.y + zoom_value, MAX_ZOOM_IN, MAX_ZOOM_OUT);
        camera_transform.scale.z = clamp(camera_transform.scale.z + zoom_value, MAX_ZOOM_IN, MAX_ZOOM_OUT);
    }
}

pub fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    camera_options: Res<CameraOptions>,
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<Camera>>
) {
    let mut camera_transform = cameras.single_mut();

    let mut direction_x = 0.;
    let mut direction_y = 0.;

    for keycode in keyboard_input.get_pressed() {
        match keycode {
            KeyCode::W => direction_y = camera_options.movement_speed,
            KeyCode::S => direction_y = -camera_options.movement_speed,
            KeyCode::A => direction_x = -camera_options.movement_speed,
            KeyCode::D => direction_x = camera_options.movement_speed,
            _ => ()
        }
    }

    camera_transform.translation.x += direction_x * time.delta_seconds();
    camera_transform.translation.y += direction_y * time.delta_seconds();
}

pub fn player_end_setup(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    placement_state: Res<State<PlacementState>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) && placement_state.get().eq(&PlacementState::NEUTRON) {
        next_game_state.set(GameState::GAME);
    }
}

pub fn player_place_neutrons(
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    atoms_q: Query<(&Transform, &Collider), With<Atom>>,
    mut placement_neutron_q: Query<(Entity, &Transform, &mut Neutron), With<PlacementMarker>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut level_stats: ResMut<LevelStats>,
    placement_state: Res<State<PlacementState>>,
    mut next_placement_state: ResMut<NextState<PlacementState>>
) {
    if level_stats.num_neutrons <= 0 || !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }
    
    let window = window_q.single();
    let (camera, camera_transform) = camera_q.single();
        
    if let Some(cursor_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
            
        if placement_state.get().eq(&PlacementState::NEUTRON) {
            for (atom_transform, atom_collider) in &atoms_q {
                let neutron_collider = Collider::new(NEUTRON_SIZE);
                
                if is_colliding((&atom_transform.translation.xy(), atom_collider), (&cursor_pos, &neutron_collider)) {
                    debug!("Selected position, {}, contains a live atom!", cursor_pos);
                    return;
                }
            }
            
            spawn_neutron_with_marker(&mut commands, &mut meshes, &mut materials, cursor_pos);
            next_placement_state.set(PlacementState::VELOCITY);

            debug!("Placed marked neutron at {}, awaiting marker...", cursor_pos);
        } else {
            let (marked_neutron_entity, marked_neutron_transform, mut marked_neutron) = placement_neutron_q.single_mut();
                
            commands.entity(marked_neutron_entity).remove::<PlacementMarker>().despawn_descendants();
            marked_neutron.velocity = clamp_vec2_by_length((cursor_pos - marked_neutron_transform.translation.xy()) * 7.5, 0.1, 1500.);
                
            level_stats.num_neutrons -= 1;
            next_placement_state.set(PlacementState::NEUTRON);
            
            debug!(
                "Set velocity of {} for neutron at {}, remaining placeable neutrons: {}", 
                marked_neutron.velocity, 
                marked_neutron_transform.translation.xy(), 
                level_stats.num_neutrons
            );
        }
    }
}

pub fn player_remove_neutron(
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    placement_neutron_q: Query<(Entity, &Transform, &Collider), With<Neutron>>,
    mut commands: Commands, 
    mouse_button_input: Res<Input<MouseButton>>,
    mut level_stats: ResMut<LevelStats>,
    placement_state: Res<State<PlacementState>>,
    mut next_placement_state: ResMut<NextState<PlacementState>>
) {
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }
    
    let window = window_q.single();
    let (camera, camera_transform) = camera_q.single();
    
    if let Some(cursor_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        
        for (neutron_entity, neutron_transform, neutron_collider) in &placement_neutron_q {
            if !is_cords_in_collider(cursor_pos, neutron_transform.translation.xy(), neutron_collider) {
                continue;
            }
            
            commands.entity(neutron_entity).despawn();
            
            if placement_state.get().eq(&PlacementState::NEUTRON) {
                level_stats.num_neutrons += 1;
            } else {
                next_placement_state.set(PlacementState::NEUTRON);
            }

            debug!("Removed neutron at {}, Remaining placeable neutrons: {}", neutron_transform.translation.xy(), level_stats.num_neutrons);
            return;
        }
    }
}