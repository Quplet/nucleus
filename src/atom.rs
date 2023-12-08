use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{collision::{Collider, is_colliding, CollisionEvent}, neutron::*};

const ATOM_SIZE: f32 = 30.;

#[derive(Component, Debug, Clone)]
pub struct Atom {
    pub num_neutrons: i32
}

pub fn spawn_atom(
    commands: &mut Commands, 
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    num_neutron: i32
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(ATOM_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            transform: Transform { translation: Vec3::from((position, 2.)), ..default() },
            ..default()
        },
        Atom { num_neutrons: num_neutron },
        Collider { radius: ATOM_SIZE * 0.75 }
    ));
}

pub fn atom_collision(
    par_commands: ParallelCommands,
    atoms: Query<(Entity, &Transform, &Atom, &Collider), Without<Neutron>>,
    neutrons: Query<(Entity, &Transform, &Neutron, &Collider), Without<Atom>>
) {
    atoms.par_iter().for_each(|(atom_entity, atom_transform, atom, atom_collider)| {
        for (neutron_entity, neutron_transform, neutron, neutron_collider) in &neutrons {
            let atom_pos = Vec2::from((atom_transform.translation.x, atom_transform.translation.y));
            let neutron_pos = Vec2::from((neutron_transform.translation.x, neutron_transform.translation.y));
            
            if !is_colliding((&atom_pos, atom_collider), (&neutron_pos, neutron_collider)) {
                continue;
            }

            par_commands.command_scope(| mut commands | {
                commands.entity(atom_entity).despawn();
                commands.entity(neutron_entity).despawn();

                let neutron_velocity = neutron.velocity;
                let num_neutrons = atom.num_neutrons;

                commands.add(move |world: &mut World| {
                    world.send_event(CollisionEvent {
                        neutron_velocity: neutron_velocity, 
                        atom_position: atom_pos,
                        num_neutrons: num_neutrons
                    });
                });
            });

            return;
        }
    });
}

