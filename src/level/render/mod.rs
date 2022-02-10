use std::ptr;
use std::time::Instant;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

use crate::level::porter::read_level;
use crate::level::render::material::{merge_materials, TEXTURE_SIZE};
use crate::level::render::mesh::{get_or_create, merge_voxels};
use crate::level::render::voxel_sequence::VoxelSequence;
use crate::Material;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};

pub mod material;
mod mesh;
mod voxel_sequence;

const PI: f32 = std::f32::consts::PI;

#[allow(clippy::needless_pass_by_value)]
pub fn init_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>,
) {
    let limits = render_device.limits().max_texture_dimension_2d;
    // This is needed because of wgpu limitation. It can't render a texture which breaks the limits in some dimension
    let max_voxels_per_dimension = limits / TEXTURE_SIZE;
    dbg!(max_voxels_per_dimension);

    let map = read_level("debug");
    let merged_voxels = merge_voxels(&map, max_voxels_per_dimension);

    let start = Instant::now();

    for sequence in &merged_voxels {
        let pos = sequence.start_position();
        let width = sequence.x_width();
        let height = sequence.y_height();

        let top_side_visible = is_top_side_visible(sequence, &merged_voxels);
        let bottom_side_visible = is_bottom_side_visible(sequence, &merged_voxels);
        let right_side_visible = is_right_side_visible(sequence, &merged_voxels);
        let left_side_visible = is_left_side_visible(sequence, &merged_voxels);
        let forward_side_visible = is_forward_side_visible(sequence, &merged_voxels);
        let back_side_visible = is_back_side_visible(sequence, &merged_voxels);

        let mut light_spawned = false;

        // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
        if top_side_visible || bottom_side_visible {
            let material = merge_materials(
                sequence.material(),
                &mut materials,
                &mut images,
                width as u32,
                height as u32,
            );

            if top_side_visible {
                let mesh = get_or_create(&mut meshes, width, height, false);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, sequence.material());
                }
            }

            if bottom_side_visible {
                let mesh = get_or_create(&mut meshes, width, height, true);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height / 2.0, pos.z - 1.0),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, sequence.material());
                }
            }
        }

        if right_side_visible || left_side_visible {
            let material = merge_materials(
                sequence.material(),
                &mut materials,
                &mut images,
                1,
                height as u32,
            );

            if right_side_visible {
                let mesh = get_or_create(&mut meshes, 1.0, height, false);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + width, pos.y + height / 2.0, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, sequence.material());
                }
            }

            if left_side_visible {
                let mesh = get_or_create(&mut meshes, 1.0, height, true);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x, pos.y + height / 2.0, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, sequence.material());
                }
            }
        }

        if forward_side_visible || back_side_visible {
            let material = merge_materials(
                sequence.material(),
                &mut materials,
                &mut images,
                width as u32,
                1,
            );

            if forward_side_visible {
                let mesh = get_or_create(&mut meshes, width, 1.0, true);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y + height, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = !light_spawned && spawn_light(&mut entity_commands, sequence.material());
                }
            }

            if back_side_visible {
                let mesh = get_or_create(&mut meshes, width, 1.0, false);
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x + width / 2.0, pos.y, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    spawn_light(&mut entity_commands, sequence.material());
                }
            }
        }
    }

    println!("world initialization: {:?}", start.elapsed());
}

fn spawn_light(entity_commands: &mut EntityCommands, material: Material) -> bool {
    if material == Material::OrangeLight {
        spawn_orange_light_source_inside(entity_commands);
        true
    } else if material == Material::BlueLight {
        spawn_blue_light_source_inside(entity_commands);
        true
    } else {
        false
    }
}

fn is_left_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (start_x, ..) = main_sequence.x_borders();

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.intersects_by_y(main_sequence)
                && sequence.has_x_end_on(start_x)
                && sequence.is_not_transparent()
                // checking if it is not the same sequence
                && !is_same_sequence(*sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !main_sequence.covered_y().all(|y| adjoining_plane_y.contains(&y))
}

fn is_right_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_x) = main_sequence.x_borders();

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.intersects_by_y(main_sequence)
                && sequence.has_x_start_on(end_x)
                && sequence.is_not_transparent()
                // checking if it is not the same sequence
                && !is_same_sequence(*sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !main_sequence.covered_y().all(|y| adjoining_plane_y.contains(&y))
}

fn is_back_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (start_y, ..) = main_sequence.y_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.has_y_end_on(start_y)
                && sequence.is_not_transparent()
                // checking if it is not the same sequence
                && !is_same_sequence(*sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence.covered_x().all(|x| adjoining_plane_x.contains(&x))
}

fn is_forward_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_y) = main_sequence.y_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.has_y_start_on(end_y)
                && sequence.is_not_transparent()
                // checking if it is not the same sequence
                && !is_same_sequence(*sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence.covered_x().all(|x| adjoining_plane_x.contains(&x))
}

fn is_bottom_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    if main_sequence.has_height(0.0) {
        return false;
    }

    let next_z_layer = get_next_z_layer(main_sequence, sequences, -1.0);

    for y in main_sequence.covered_y() {
        for x in main_sequence.covered_x() {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

fn is_top_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    let next_z_layer = get_next_z_layer(main_sequence, sequences, 1.0);

    for y in main_sequence.covered_y() {
        for x in main_sequence.covered_x() {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

fn get_next_z_layer<'a>(main_sequence: &'a VoxelSequence, all_sequences: &'a [VoxelSequence], z_bonus: f32) -> Vec<(usize, usize)> {
    let height = main_sequence.height() + z_bonus;

    all_sequences.iter()
        .filter(|sequence| {
            sequence.has_height(height)
                && sequence.intersects_by_y(main_sequence)
                && sequence.intersects_by_x(main_sequence)
                && sequence.is_not_transparent()
        })
        .flat_map(VoxelSequence::covered_coordinates)
        .collect()
}

fn is_same_sequence(a: &VoxelSequence, b: &VoxelSequence) -> bool {
    // checking if pointer points to the same struct
    ptr::eq(a, b)
}