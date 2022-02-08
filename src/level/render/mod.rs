use std::time::Instant;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;

use crate::entity::point::Point;
use crate::entity::voxel::Voxel;
use crate::level::porter::read_level;
use crate::level::render::material::{merge_materials, TEXTURE_SIZE};
use crate::level::render::mesh::{merge_voxels, VoxelSequence};
use crate::Material;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};

pub mod material;
pub mod mesh;

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

        let top_side_visible = is_top_side_visible(pos, &box_shape, &concatenated_voxels);
        let bottom_side_visible = is_bottom_side_visible(pos, &box_shape, &concatenated_voxels);
        let right_side_visible = is_right_side_visible(pos, &box_shape, &concatenated_voxels);
        let left_side_visible = is_left_side_visible(pos, &box_shape, &concatenated_voxels);
        let forward_side_visible = is_forward_side_visible(pos, &box_shape, &concatenated_voxels);
        let back_side_visible = is_back_side_visible(pos, &box_shape, &concatenated_voxels);

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

fn is_left_side_visible(pos: &Point, shape: &shape::Box, all_shapes: &[VoxelSequence]) -> bool {
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;
    let min_x = pos.x;

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            let x_ends_on_the_start = sequence.has_x_end_on(min_x);
            let same_height = sequence.has_height(pos.z);
            let start_y_within_borders = sequence.contains_y(min_y);
            let end_y_within_borders = sequence.contains_y(max_y);
            let is_not_transparent = sequence.is_not_transparent();

            same_height
                && (start_y_within_borders || end_y_within_borders)
                && x_ends_on_the_start
                && is_not_transparent
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !(min_y as usize..=max_y as usize).into_iter().all(|y| adjoining_plane_y.contains(&y))
}

fn is_right_side_visible(pos: &Point, shape: &shape::Box, all_shapes: &[VoxelSequence]) -> bool {
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;
    let max_x = pos.x + shape.max_x;

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            let same_height = sequence.has_height(pos.z);
            let x_starts_on_the_end = sequence.has_x_start_on(max_x);
            let start_y_within_borders = sequence.contains_y(min_y);
            let end_y_within_borders = sequence.contains_y(max_y);
            let is_not_transparent = sequence.is_not_transparent();

            same_height
                && (start_y_within_borders || end_y_within_borders)
                && x_starts_on_the_end
                && is_not_transparent
        })
        .flat_map(VoxelSequence::covered_y)
        .collect();

    !(min_y as usize..=max_y as usize).into_iter().all(|y| adjoining_plane_y.contains(&y))
}

fn is_back_side_visible(pos: &Point, shape: &shape::Box, all_shapes: &[VoxelSequence]) -> bool {
    let min_x = pos.x as usize;
    let max_x = (pos.x + shape.max_x) as usize;

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            let same_height = sequence.has_height(pos.z);
            let ends_on_the_start = sequence.has_y_end_on(pos.y);
            let is_not_transparent = sequence.is_not_transparent();

            same_height && ends_on_the_start
                && is_not_transparent
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !(min_x..=max_x).into_iter().all(|x| adjoining_plane_x.contains(&x))
}

fn is_forward_side_visible(pos: &Point, shape: &shape::Box, all_shapes: &[VoxelSequence]) -> bool {
    let min_x = pos.x as usize;
    let max_x = (pos.x + shape.max_x) as usize;
    let max_y = pos.y + shape.max_y;

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|sequence| {
            let same_height = sequence.has_height(pos.z);
            let starts_on_the_end = sequence.has_y_start_on(max_y);
            let is_not_transparent = sequence.is_not_transparent();

            same_height && starts_on_the_end && is_not_transparent
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !(min_x..=max_x).into_iter().all(|x| adjoining_plane_x.contains(&x))
}

fn is_bottom_side_visible(pos: &Point, shape: &shape::Box, sequences: &[VoxelSequence]) -> bool {
    if pos.z == 0.0 {
        return false;
    }

    let min_x = pos.x;
    let max_x = pos.x + shape.max_x;
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;

    let next_z_layer = get_next_z_layer(pos, shape, sequences, -1.0);

    for y in min_y as usize..max_y as usize {
        for x in min_x as usize..max_x as usize {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

// TODO fix random bug of unneeded rendering
fn is_top_side_visible(pos: &Point, shape: &shape::Box, sequences: &[VoxelSequence]) -> bool {
    let min_x = pos.x;
    let max_x = pos.x + shape.max_x;
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;

    let next_z_layer = get_next_z_layer(pos, shape, sequences, 1.0);

    for y in min_y as usize..max_y as usize {
        for x in min_x as usize..max_x as usize {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

fn get_next_z_layer<'a>(pos: &'a Point, shape: &'a shape::Box, sequences: &'a [VoxelSequence], z_bonus: f32) -> Vec<(usize, usize)> {
    let start_x = pos.x;
    let end_x = pos.x + shape.max_x;
    let start_y = pos.y;
    let end_y = pos.y + shape.max_y;

    sequences.iter()
        .filter(|sequence| {
            let right_height = sequence.has_height(pos.z + z_bonus);
            let column_start_within_borders = sequence.contains_y(start_y);
            let column_end_within_borders = sequence.contains_y(end_y);
            let row_start_within_borders = sequence.contains_x(start_x);
            let row_end_within_borders = sequence.contains_x(end_x);
            let is_not_transparent = sequence.is_not_transparent();

            right_height
                && (column_start_within_borders || column_end_within_borders)
                && (row_start_within_borders || row_end_within_borders)
                && is_not_transparent
        })
        .flat_map(VoxelSequence::covered_coordinates)
        .collect()
}

