use atom::atom_cleanup;
use atom::atom_collision;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use collision::CollisionEvent;
use collision::collision_listener;
use hud::hud_cleanup;
use hud::hud_setup;
use hud::hud_text_update;
use hud::reset_button;
use hud::setup_reset_button;
use level_manager::setup_level;
use bevy::prelude::*;
use bevy::log::*;
use menu::button_system;
use menu::main_menu_cleanup;
use neutron::neutron_cleanup;
use neutron::neutron_motion;
use neutron::pointer_follow_cursor;
use player_controls::*;
use menu::main_menu_setup;

mod atom;
mod neutron;
pub mod collision;
mod player_controls;
mod level_manager;
mod util;
mod hud;
mod menu;

const ENERGY_RELEASED: f64 = 3.2e-11;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    MENU,
    SETUP,
    GAME,
    PAUSED
}

#[derive(Resource, Debug)]
pub struct GameStats {
    pub score: f64,
    pub level: i32,
    pub simulation_speed: f32
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats { score: 0., level: 0, simulation_speed: 0.5 }
    }
}

fn main_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
    .add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Nucleus".into(),
            ..default()
        }),
        ..default()
    }).set(LogPlugin {
        // uncomment to change log levels
        level: Level::DEBUG,
        ..default()
    }), FrameTimeDiagnosticsPlugin))

    .add_state::<GameState>()
    .add_state::<PlacementState>()
    .insert_resource(GameStats::default())
    .insert_resource(CameraOptions::default())

    // background color
    .insert_resource(ClearColor(Color::BLACK))

    .add_event::<CollisionEvent>()

    .add_systems(Startup, main_setup)

    .add_systems(OnEnter(GameState::MENU), main_menu_setup)
    .add_systems(OnExit(GameState::MENU), main_menu_cleanup)
    
    .add_systems(OnEnter(GameState::SETUP), (setup_level, hud_setup))

    .add_systems(OnEnter(GameState::GAME), setup_reset_button)
    .add_systems(OnExit(GameState::GAME), (hud_cleanup, neutron_cleanup, atom_cleanup))
    
    .add_systems(Update, button_system.run_if(in_state(GameState::MENU)))

    .add_systems(Update, (player_end_setup, player_place_neutrons, player_remove_neutron, pointer_follow_cursor).run_if(in_state(GameState::SETUP)))

    .add_systems(Update, (neutron_motion, atom_collision, collision_listener, reset_button).run_if(in_state(GameState::GAME)))

    .add_systems(Update, (camera_zoom, camera_movement, hud_text_update).run_if(in_state(GameState::GAME).or_else(in_state(GameState::SETUP))))

    .run();
}
