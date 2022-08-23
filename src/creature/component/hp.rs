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
    creatures: Query<Entity, (With<(EnemyCreatureMarker)>, Added<HP>)>,
) {
    let hp_material = materials.add(StandardMaterial {
        base_color: Color::ORANGE_RED,
        unlit: true,
        ..Default::default()
    });
    let hp_mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.4,
        ..Default::default()
    }));

    for creature in creatures.iter() {
        commands.entity(creature).with_children(|builder| {
            builder
                .spawn_bundle(PbrBundle {
                    mesh: hp_mesh.clone(),
                    material: hp_material.clone(),
                    transform: Transform::from_xyz(0., 4.5, 0.),
                    ..Default::default()
                })
                .insert(HPMeshMarker);
        });
    }
}

pub fn creature_hp_change_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hp_materials: Query<(&mut Handle<StandardMaterial>, &Parent), With<HPMeshMarker>>,
    hps: Query<&HP>,
) {
    for (mut material, parent) in hp_materials.iter_mut() {
        if let Ok(hp) = hps.get(**parent) {
            let p = hp.percent();
            let mut current_hp_material = materials.get(&material).unwrap().clone();
            let new_color = Color::rgb(
                0.5 * (p * 2.),
                current_hp_material.base_color.g(),
                0.3 * (1. - p),
            );

            if current_hp_material.base_color != new_color {
                dbg!("changing color");
                current_hp_material.base_color = Color::rgb(
                    0.5 * (p * 2.),
                    current_hp_material.base_color.g(),
                    0.3 * (1. - p),
                );
                let handle = materials.add(current_hp_material);
                *material = handle;
            }
        }
    }
    materials.set_changed();
}
