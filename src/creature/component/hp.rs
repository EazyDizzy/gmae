use crate::creature::EnemyCreatureMarker;
use bevy::prelude::*;
use std::cmp;

#[derive(Component, Debug)]
pub struct HPMeshMarker;

#[derive(Component, Debug)]
pub struct HP {
    max: u16,
    current: u16,
}

impl HP {
    pub fn full(max: u16) -> HP {
        HP { max, current: max }
    }
    pub fn percent(&self) -> f32 {
        f32::from(self.current) / f32::from(self.max)
    }

    pub fn max(&self) -> u16 {
        self.max
    }
    pub fn current(&self) -> u16 {
        self.current
    }
    pub fn is_empty(&self) -> bool {
        self.current == 0
    }

    pub fn sub(&mut self, amount: u16) {
        self.current -= cmp::min(self.current, amount);
    }
}

pub fn creature_hp_spawn_mesh(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    creatures: Query<Entity, (With<EnemyCreatureMarker>, Added<HP>)>,
) {
    let black_hp_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        unlit: true,
        ..Default::default()
    });
    let red_hp_material = materials.add(StandardMaterial {
        base_color: Color::RED,
        unlit: true,
        ..Default::default()
    });
    let red_hp_mesh = meshes.add(Mesh::from(shape::Box::new(2.01, 0.51, 0.51)));
    let black_hp_mesh = meshes.add(Mesh::from(shape::Box::new(2.0, 0.5, 0.5)));

    for creature in creatures.iter() {
        commands.entity(creature).with_children(|builder| {
            builder
                .spawn_bundle(PbrBundle {
                    mesh: red_hp_mesh.clone(),
                    material: red_hp_material.clone(),
                    transform: Transform::from_xyz(0., 4.5, 0.),
                    ..Default::default()
                })
                .insert(HPMeshMarker);
            builder.spawn_bundle(PbrBundle {
                mesh: black_hp_mesh.clone(),
                material: black_hp_material.clone(),
                transform: Transform::from_xyz(0., 4.5, 0.),
                ..Default::default()
            });
        });
    }
}

// TODO lock hp rotation to camera
pub fn creature_hp_mesh_change_percent(
    mut hp_meshes: Query<(&Parent, &mut Transform), With<HPMeshMarker>>,
    hps: Query<&HP>,
) {
    for (parent, mut transform) in hp_meshes.iter_mut() {
        if let Ok(hp) = hps.get(**parent) {
            let p = hp.percent();
            if transform.scale.x != p {
                transform.scale.x = p;
                transform.translation.x = 1. - p;
            }
        }
    }
}
