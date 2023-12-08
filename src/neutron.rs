use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{collision::*, GameStats};

pub const NEUTRON_SIZE: f32 = 10.;
const NEUTRON_COLOR: Color = Color::rgb(0.3, 0.3, 1.0);

#[derive(Component, Debug, Clone)]
pub struct Neutron {
    pub velocity: Vec2
}

#[derive(Component)]
pub struct PlacementMarker;

#[derive(Component, Default)]
pub struct PlacementPointer {
    set_vel: bool
}

pub fn neutron_motion(
    mut neutrons: Query<(&mut Transform, &Neutron)>,
    time: Res<Time>,
    game_stats: Res<GameStats>
) {
    neutrons.par_iter_mut().for_each(|(mut neutron_transform, neutron)| {
        neutron_transform.translation += Vec3::from((neutron.velocity * time.delta_seconds() * game_stats.simulation_speed, 0.));
    });
}

pub fn spawn_neutron(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2, 
    velocity: Vec2
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(NEUTRON_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(NEUTRON_COLOR)),
            transform: Transform {
                translation: Vec3::from((position, 1.)),
                ..default()
            },
            ..default()
        },
        Neutron { velocity: velocity },
        Collider { radius: NEUTRON_SIZE * 0.75 }
    ));
}

pub fn spawn_neutron_with_marker(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(NEUTRON_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(NEUTRON_COLOR)),
            transform: Transform {
                translation: Vec3::from((position, 1.)),
                ..default()
            },
            ..default()
        },
        Neutron { velocity: Vec2::ZERO },
        Collider { radius: NEUTRON_SIZE * 0.75 },
        PlacementMarker
    )).with_children(|parent| {
        parent.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad { size: Vec2::new(40., NEUTRON_SIZE/2.), flip: false }.into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform {
                    translation: Vec3::new(20., 0., -1.),
                    ..default()
                },
                ..default()
            },
            PlacementPointer::default()
        ));
    });
}

/*
    a lot taken https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs
*/
pub fn pointer_follow_cursor(
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut pointer_q: Query<(&mut Transform, &PlacementPointer, &Parent)>,
    parent_q: Query<&GlobalTransform, (With<Neutron>, With<PlacementMarker>)>
) {
    let window = window_q.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(cursor_pos) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        for (mut pointer_transform, placement_pointer, pointer_parent) in &mut pointer_q {
            if placement_pointer.set_vel {
                continue;
            }

            // unwrap, if there is a parent in the q that doesn't have this something has gone horribly wrong.
            let parent_global_transform = parent_q.get(pointer_parent.get()).unwrap(); 

            let to_cursor = cursor_pos - parent_global_transform.translation().xy();

            if to_cursor.length() < f32::EPSILON {
                return;
            }

            let pointer_right = (pointer_transform.rotation * Vec3::Y).xy();
            let rotation_angle = pointer_right.dot(to_cursor.normalize());

            if rotation_angle.abs() < f32::EPSILON {
                return;
            }

            pointer_transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(rotation_angle));
        }
    }
}

pub fn calculate_split_trajectories(neutron_velocity: Vec2, num_split: i32) -> Option<Vec<Vec2>> {
    let normalized_velocity = neutron_velocity.normalize();

    let vec = match num_split {
        1 => {
            vec![neutron_velocity]
        },
        2 => {
            let rotation1 = Vec2::from_angle(PI/4.);
            let rotation2 = Vec2::from_angle((7. * PI)/4.);

            vec![
                rotation1.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation2.rotate(normalized_velocity) * neutron_velocity.length()
            ]
        },
        3 => {
            let rotation1 = Vec2::from_angle(PI/3.);
            let rotation2 = Vec2::from_angle(0.);
            let rotation3 = Vec2::from_angle((5. * PI)/3.);

            vec![
                rotation1.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation2.rotate(normalized_velocity) * neutron_velocity.length(),
                rotation3.rotate(normalized_velocity) * neutron_velocity.length()
            ]
        },
        4 => {
            let rotation1 = Vec2::from_angle(PI/3.);
            let rotation2 = Vec2::from_angle(PI/6.);
            let rotation3 = Vec2::from_angle((11. * PI)/6.);
            let rotation4 = Vec2::from_angle((5. * PI)/3.);

            vec![
                rotation1.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation2.rotate(normalized_velocity) * neutron_velocity.length(),
                rotation3.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation4.rotate(normalized_velocity) * neutron_velocity.length()
            ]
        },
        5 => {
            let rotation1 = Vec2::from_angle(PI/3.);
            let rotation2 = Vec2::from_angle(PI/6.);
            let rotation3 = Vec2::from_angle(0.);
            let rotation4 = Vec2::from_angle((11. * PI)/6.);
            let rotation5 = Vec2::from_angle((5. * PI)/3.);

            vec![
                rotation1.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation2.rotate(normalized_velocity) * neutron_velocity.length(),
                rotation3.rotate(normalized_velocity) * neutron_velocity.length(), 
                rotation4.rotate(normalized_velocity) * neutron_velocity.length(),
                rotation5.rotate(normalized_velocity) * neutron_velocity.length()
            ]
        }
        _ => {
            error!("Too many neutrons/negative neutrons! {}", num_split);
            return None;
        }
    };

    Some(vec)
}