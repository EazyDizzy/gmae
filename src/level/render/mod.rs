use std::time::Instant;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

use crate::entity::voxel::Voxel;
use crate::level::porter::read_level;
use crate::level::render::material::{merge_materials, TEXTURE_SIZE};
use crate::level::render::mesh::merge_voxels;
use crate::level::render::voxel_sequence::VoxelSequence;
use crate::Material;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};

pub mod material;
pub mod mesh;
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

    let concatenated_voxels = merge_voxels(&map, max_voxels_per_dimension);
    let start = Instant::now();

    for sequence in &concatenated_voxels {
        let start_voxel = sequence.start;
        let box_shape = sequence.get_box();
        let pos = &start_voxel.position;
        let x_size = box_shape.max_x;
        let y_size = box_shape.max_y;

        let top_side_visible = is_top_side_visible(sequence, &concatenated_voxels);
        let bottom_side_visible = is_bottom_side_visible(sequence, &concatenated_voxels);
        let right_side_visible = is_right_side_visible(sequence, &concatenated_voxels);
        let left_side_visible = is_left_side_visible(sequence, &concatenated_voxels);
        let forward_side_visible = is_forward_side_visible(sequence, &concatenated_voxels);
        let back_side_visible = is_back_side_visible(sequence, &concatenated_voxels);

        let mut light_spawned = false;

        // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
        if top_side_visible || bottom_side_visible {
            let material = merge_materials(
                start_voxel.material,
                &mut materials,
                &mut images,
                x_size as u32,
                y_size as u32,
            );

            if top_side_visible {
                let shape = shape::Quad { size: Vec2::new(x_size, y_size), flip: false };
                let mesh = meshes.add(Mesh::from(shape));

                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size / 2.0, pos.z),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, start_voxel);
                }
            }

            if bottom_side_visible {
                let shape = shape::Quad { size: Vec2::new(x_size, y_size), flip: true };
                let mesh = meshes.add(Mesh::from(shape));
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size / 2.0, pos.z - 1.0),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, start_voxel);
                }
            }
        }

        if right_side_visible || left_side_visible {
            let material = merge_materials(
                start_voxel.material,
                &mut materials,
                &mut images,
                1,
                y_size as u32,
            );

            if right_side_visible {
                let shape = shape::Quad { size: Vec2::new(1.0, y_size), flip: false };
                let mesh = meshes.add(Mesh::from(shape));
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + x_size, pos.y + y_size / 2.0, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, start_voxel);
                }
            }

            if left_side_visible {
                let shape = shape::Quad { size: Vec2::new(1.0, y_size), flip: true };
                let mesh = meshes.add(Mesh::from(shape));
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x, pos.y + y_size / 2.0, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = spawn_light(&mut entity_commands, start_voxel);
                }
            }
        }

        if forward_side_visible || back_side_visible {
            let material = merge_materials(
                start_voxel.material,
                &mut materials,
                &mut images,
                x_size as u32,
                1,
            );

            if forward_side_visible {
                let shape = shape::Quad { size: Vec2::new(x_size, 1.0), flip: true };
                let mesh = meshes.add(Mesh::from(shape));
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    light_spawned = !light_spawned && spawn_light(&mut entity_commands, start_voxel);
                }
            }

            if back_side_visible {
                let shape = shape::Quad { size: Vec2::new(x_size, 1.0), flip: false };
                let mesh = meshes.add(Mesh::from(shape));
                let mut entity_commands = commands.spawn_bundle(PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y, pos.z - 0.5)
                        .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                    ..Default::default()
                });

                if !light_spawned {
                    spawn_light(&mut entity_commands, start_voxel);
                }
            }
        }
    }

    println!("world initialization: {:?}", start.elapsed());
}

fn spawn_light(entity_commands: &mut EntityCommands, voxel: &Voxel) -> bool {
    if voxel.material == Material::OrangeLight {
        spawn_orange_light_source_inside(entity_commands);
        true
    } else if voxel.material == Material::BlueLight {
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
            sequence.has_height(main_sequence.height())
                && sequence.intersects_by_y(main_sequence)
                && sequence.has_x_end_on(start_x)
                && sequence.is_not_transparent()
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !main_sequence.covered_y().all(|y| adjoining_plane_y.contains(&y))
}

fn is_right_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_x) = main_sequence.x_borders();

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_height(main_sequence.height())
                && sequence.intersects_by_y(main_sequence)
                && sequence.has_x_start_on(end_x)
                && sequence.is_not_transparent()
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !main_sequence.covered_y().all(|y| adjoining_plane_y.contains(&y))
}

fn is_back_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (start_y, ..) = main_sequence.y_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_height(main_sequence.height())
                && sequence.has_y_end_on(start_y)
                && sequence.is_not_transparent()
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence.covered_x().all(|x| adjoining_plane_x.contains(&x))
}

fn is_forward_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_y) = main_sequence.y_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            sequence.has_height(main_sequence.height())
                && sequence.has_y_start_on(end_y)
                && sequence.is_not_transparent()
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence.covered_x().all(|x| adjoining_plane_x.contains(&x))
}

fn is_bottom_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    if main_sequence.has_height(0.0) {
        return false;
    }

    let (start_x, end_x) = main_sequence.x_borders();
    let (start_y, end_y) = main_sequence.y_borders();

    let next_z_layer = get_next_z_layer(main_sequence, sequences, -1.0);

    for y in start_y as usize..end_y as usize {
        for x in start_x as usize..end_x as usize {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

// TODO fix random bug of unneeded rendering
fn is_top_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    let (start_x, end_x) = main_sequence.x_borders();
    let (start_y, end_y) = main_sequence.y_borders();
    let next_z_layer = get_next_z_layer(main_sequence, sequences, 1.0);

    for y in start_y as usize..end_y as usize {
        for x in start_x as usize..end_x as usize {
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

