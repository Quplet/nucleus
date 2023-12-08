use bevy::prelude::*;

use crate::{neutron::{calculate_split_trajectories, spawn_neutron}, GameStats, ENERGY_RELEASED};

#[derive(Component, Debug, Default, Clone)]
pub struct Collider {
    pub radius: f32
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider {
            radius: radius
        }
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub neutron_velocity: Vec2,
    pub atom_position: Vec2,
    pub num_neutrons: i32
}

pub fn collision_listener(
    mut events: EventReader<CollisionEvent>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_stats: ResMut<GameStats>
) {
    for collision_event in events.read() {
        debug!(
            "Collison! Neutron velocity: {}, Atom position: {}, Num neutrons: {}", 
            collision_event.neutron_velocity, 
            collision_event.atom_position, 
            collision_event.num_neutrons
        );

        if let Some(new_vectors) = calculate_split_trajectories(collision_event.neutron_velocity, collision_event.num_neutrons) {
            for vector in new_vectors {
                spawn_neutron(&mut commands, &mut meshes, &mut materials, collision_event.atom_position, vector);
            }
        }

        game_stats.score += ENERGY_RELEASED;
    }
}

pub fn is_colliding(pair1: (&Vec2, &Collider), pair2: (&Vec2, &Collider)) -> bool {
    (pair1.0.to_owned() - pair2.0.to_owned()).length() < pair1.1.radius + pair2.1.radius
}

pub fn is_cords_in_collider(cords: Vec2, collider_pos: Vec2, collider: &Collider) -> bool {
    (collider_pos - cords).length() < collider.radius
}