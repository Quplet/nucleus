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
    pub s_score: f64
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

pub fn calculate_grade(score: f64, level_stats: &LevelStats) -> &'static str {
    let letters = ["S", "A", "B", "C", "D"];

    for i in 0..letters.len() {
        if score + f64::EPSILON >= level_stats.s_score * 0.5 + level_stats.s_score * 1./(2. + i as f64) {
            return letters[i];
        }
    }

    "F"
}

fn get_level(level: i32) -> Option<Level> {
    match level {
        0 => {
            Some(Level {
                //level: 0,
                atoms: vec![(2, Vec2::new(0., 0.)), (3, Vec2::new(-100., 100.)), (3, Vec2::new(100., 100.))],
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 3. }
            })
        }
        _ => { None }
    }
}