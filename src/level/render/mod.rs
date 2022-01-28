use std::collections::{HashMap, HashSet};
use std::time::Instant;

use bevy::prelude::*;

use crate::entity::voxel::{Voxel, VoxelMaterial};
use crate::level::porter::read_level;
use crate::level::render::material::get_material;
use crate::level::render::mesh::get_entity_mesh;
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

pub mod material;
pub mod mesh;

pub fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    let map = read_level("debug");

    let now = Instant::now();
    let (concatenated_voxels, not_used_voxels) = concatenate_voxels(&map);
    let time = now.elapsed().as_millis();
    println!("concatenate_voxels time {}ms", time);
    println!("concatenations {}", concatenated_voxels.len());
    println!("unused voxels {}", not_used_voxels.len());
    for (mesh_form, voxel) in concatenated_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = meshes.add(mesh_form);

        commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });
    }

    for voxel in not_used_voxels {
        let pos = &voxel.position;
        let material = get_material(voxel.material, &materials);
        let mesh = get_entity_mesh(voxel.material, &meshes);

        let mut entity_commands = commands.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });

        if voxel.material == VoxelMaterial::OrangeLight {
            spawn_orange_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
        if voxel.material == VoxelMaterial::BlueLight {
            spawn_blue_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
    }
}

fn concatenate_voxels(voxels: &Vec<Voxel>) -> (Vec<(Mesh, &Voxel)>, Vec<&Voxel>) {
    let mut stats = HashMap::new();
    let mut all_voxels = HashSet::new();

    for voxel in voxels {
        let z = voxel.position.z.round().to_string();
        if stats.get(&z).is_none() {
            stats.insert(z.clone(), vec![]);
        }
        all_voxels.insert(voxel.id);

        stats.get_mut(&z).unwrap().push(voxel);
    }

    let mut meshes = vec![];

    for (_, row) in stats {
        for voxel in &row {
            let neighbours: Vec<&&Voxel> = row.iter().filter(|b| {
                if !all_voxels.contains(&b.id) {
                    return false;
                }
                if b.material != voxel.material {
                    return false;
                }

                let x_diff = (b.position.x as usize).abs_diff(voxel.position.x as usize);
                let y_diff = (b.position.y as usize).abs_diff(voxel.position.y as usize);

                x_diff <= 1 && y_diff <= 1
            }).collect();

            if neighbours.len() == 9 {
                for neighbour in &neighbours {
                    all_voxels.remove(&neighbour.id);
                }
                meshes.push((Mesh::from(
                    shape::Box::new(3.0, 3.0, 1.0)
                ), *voxel));
            }
        }
    }

    let not_used_voxels: Vec<&Voxel> = voxels.iter().filter(|v| all_voxels.contains(&v.id)).collect();

    (meshes, not_used_voxels)
}
