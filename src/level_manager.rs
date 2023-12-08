use bevy::prelude::*;

use crate::{atom::*, GameStats};

pub struct Level {
    //level: i32,
    atoms: Vec<(i32, Vec2)>,
    level_stats: LevelStats
}

#[derive(Resource, Debug)]
pub struct LevelStats {
    pub num_neutrons: i32,
    pub passing_score: f64
}

pub fn setup_level(
    mut commands: Commands,
    game_stats: Res<GameStats>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if let Some(level) = get_level(game_stats.level) {
        for (atom_neutrons, position) in level.atoms {
            spawn_atom(&mut commands, &mut meshes, &mut materials, position, atom_neutrons);
        }
        
        commands.insert_resource(level.level_stats);
    }
}

fn get_level(level: i32) -> Option<Level> {
    match level {
        0 => {
            Some(Level {
                //level: 0,
                atoms: vec![(3, Vec2::from((0., 0.)))],
                level_stats: LevelStats { num_neutrons: 1, passing_score: 3.2e-11 }
            })
        }
        _ => { None }
    }
}