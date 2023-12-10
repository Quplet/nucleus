use bevy::prelude::*;

use crate::{atom::*, GameStats};

#[derive(Component)]
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
    asset_server: Res<AssetServer>,
    mut game_stats: ResMut<GameStats>,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if let Some(level) = get_level(game_stats.level) {
        for (atom_neutrons, position) in level.atoms {
            spawn_atom(&mut commands, &asset_server, &mut meshes, &mut materials, position, atom_neutrons);
        }
        
        commands.insert_resource(level.level_stats);
        game_stats.score = 0.;
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

pub fn get_level(level: i32) -> Option<Level> {
    match level {
        0 => {
            Some(Level {
                //level: 0,
                atoms: vec![(2, Vec2::new(0., 0.)), (3, Vec2::new(-100., 100.)), (3, Vec2::new(100., 100.))],
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 3. }
            })
        }
        1 => {
            Some(Level {
                atoms: vec![
                    (5, Vec2::new(0., 0.)), 
                    (1, Vec2::new(-165., 100.)), (1, Vec2::new(165., 100.)),
                    (2, Vec2::new(-315., 200.)), (3, Vec2::new(0., 200.)), (2, Vec2::new(315., 200.)),
                ],
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 6. }
            })
        }
        2 => {
            Some(Level {
                atoms: vec![
                    (5, Vec2::new(0., 0.)), 
                    (3, Vec2::new(-75., 75.)), (3, Vec2::new(0., 75.)), (3, Vec2::new(75., 75.)),
                    (3, Vec2::new(-225., 150.)), (3, Vec2::new(-150., 150.)), (3, Vec2::new(0., 150.)), (3, Vec2::new(150., 150.)), (3, Vec2::new(225., 150.)),
                    (3, Vec2::new(-225., 225.)), (3, Vec2::new(-150., 225.)), (3, Vec2::new(0., 225.)), (3, Vec2::new(150., 225.)), (3, Vec2::new(225., 225.))
                ],
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 14. }
            })
        }
        3 => {
            Some(Level {
                atoms: vec![
                    (5, Vec2::new(0., 0.)), 
                    (3, Vec2::new(-150., 75.)), (3, Vec2::new(-75., 75.)), (3, Vec2::new(0., 75.)), (3, Vec2::new(75., 75.)), (3, Vec2::new(150., 75.)),
                    (3, Vec2::new(-225., 150.)), (3, Vec2::new(-150., 150.)), (3, Vec2::new(0., 150.)), (3, Vec2::new(150., 150.)), (3, Vec2::new(225., 150.)),
                    (3, Vec2::new(-225., 225.)), (3, Vec2::new(-150., 225.)), (3, Vec2::new(-75., 225.)), (3, Vec2::new(0., 225.)), (3, Vec2::new(75., 225.)), (3, Vec2::new(150., 225.)), (3, Vec2::new(225., 225.))
                ],
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 17. }
            })
        }
        4 => {
            Some(Level {
                atoms: generate_atom_sphere(8000, 75, 2),
                level_stats: LevelStats { num_neutrons: 1, s_score: 3.2e-11 * 35000. }
            })
        }
        _ => { None }
    }
}

fn generate_atom_sphere(radius: i64, distance_apart: usize, num_neutrons: i32) -> Vec<(i32, Vec2)> {
    let mut vec = Vec::new();

    for x in (-radius..radius).step_by(distance_apart) {
        for y in (-radius..radius).step_by(distance_apart) {
            if x.pow(2) + y.pow(2) > radius.pow(2) {
                continue;
            }

            vec.push((num_neutrons, Vec2::new(x as f32, y as f32)));
        }
    }

    vec
}