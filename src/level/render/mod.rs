use bevy::prelude::*;

use crate::entity::point::Point;
use crate::entity::voxel::Voxel;
use crate::level::porter::read_level;
use crate::level::render::material::concatenate_material;
use crate::level::render::mesh::merge_voxels_in_meshes;
use crate::system::light::{spawn_blue_light_source_inside, spawn_orange_light_source_inside};
use crate::VoxelMaterial;

pub mod material;
pub mod mesh;

const PI: f32 = std::f32::consts::PI;

pub fn render_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let map = read_level("debug");

    let concatenated_voxels = merge_voxels_in_meshes(&map);

    for (shape, voxel) in &concatenated_voxels {
        let pos = &voxel.position;
        let x_size = shape.max_x;
        let y_size = shape.max_y;

        // TODO not generate material if it is not needed
        let top_and_bottom_material = concatenate_material(
            voxel.material,
            &mut materials,
            &mut images,
            &asset_server,
            x_size as u32,
            y_size as u32,
        );
        let back_and_forward_material = concatenate_material(
            voxel.material,
            &mut materials,
            &mut images,
            &asset_server,
            x_size as u32,
            1,
        );
        let left_and_right_material = concatenate_material(
            voxel.material,
            &mut materials,
            &mut images,
            &asset_server,
            1,
            y_size as u32,
        );

        let top_side_needed = is_top_side_needed(pos, shape, &concatenated_voxels);
        let bottom_side_needed = is_bottom_side_needed(pos, shape, &concatenated_voxels);
        let right_side_needed = is_right_side_needed(pos, shape, &concatenated_voxels);
        let left_side_needed = is_left_side_needed(pos, shape, &concatenated_voxels);
        let forward_side_needed = is_forward_side_needed(pos, shape, &concatenated_voxels);
        let back_side_needed = is_back_side_needed(pos, shape, &concatenated_voxels);

        if top_side_needed {
            let top_shape = shape::Quad {
                size: Vec2::new(x_size, y_size),
                flip: false,
            };
            let top_mesh = meshes.add(Mesh::from(top_shape));

            // Quad shapes use transform translation coordinates as their center. That's why a bonus of size/2 is added
            let mut entity_commands = commands.spawn_bundle(PbrBundle {
                mesh: top_mesh,
                material: top_and_bottom_material.clone(),
                transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size / 2.0, pos.z),
                ..Default::default()
            });

            // TODO should not depend on top side
            if voxel.material == VoxelMaterial::OrangeLight {
                spawn_orange_light_source_inside(&mut entity_commands);
            }
            if voxel.material == VoxelMaterial::BlueLight {
                spawn_blue_light_source_inside(&mut entity_commands);
            }
        }

        if bottom_side_needed {
            let bottom_shape = shape::Quad {
                size: Vec2::new(x_size, y_size),
                flip: true,
            };
            let bottom_mesh = meshes.add(Mesh::from(bottom_shape));
            commands.spawn_bundle(PbrBundle {
                mesh: bottom_mesh,
                material: top_and_bottom_material.clone(),
                transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size / 2.0, pos.z - 1.0),
                ..Default::default()
            });
        }

        if right_side_needed {
            let right_shape = shape::Quad {
                size: Vec2::new(1.0, y_size),
                flip: false,
            };
            let right_mesh = meshes.add(Mesh::from(right_shape));
            commands.spawn_bundle(PbrBundle {
                mesh: right_mesh,
                material: left_and_right_material.clone(),
                transform: Transform::from_xyz(pos.x + x_size, pos.y + y_size / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }

        if left_side_needed {
            let left_shape = shape::Quad {
                size: Vec2::new(1.0, y_size),
                flip: true,
            };
            let left_mesh = meshes.add(Mesh::from(left_shape));
            commands.spawn_bundle(PbrBundle {
                mesh: left_mesh,
                material: left_and_right_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y + y_size / 2.0, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, 0.0)),
                ..Default::default()
            });
        }

        if forward_side_needed {
            let forward_shape = shape::Quad {
                size: Vec2::new(x_size, 1.0),
                flip: true,
            };
            let forward_mesh = meshes.add(Mesh::from(forward_shape));
            commands.spawn_bundle(PbrBundle {
                mesh: forward_mesh,
                material: back_and_forward_material.clone(),
                transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y + y_size, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }

        if back_side_needed {
            let back_shape = shape::Quad {
                size: Vec2::new(x_size, 1.0),
                flip: false,
            };
            let back_mesh = meshes.add(Mesh::from(back_shape));
            commands.spawn_bundle(PbrBundle {
                mesh: back_mesh,
                material: back_and_forward_material.clone(),
                transform: Transform::from_xyz(pos.x + x_size / 2.0, pos.y, pos.z - 0.5)
                    .with_rotation(Quat::from_euler(EulerRot::XYZ, PI / 2.0, 0.0, 0.0)),
                ..Default::default()
            });
        }
    }
}

fn is_left_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;
    let min_x = pos.x;

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|(s, v)| {
            let seq_max_x = v.position.x + s.max_x;
            let seq_min_y = v.position.y;
            let seq_max_y = v.position.y + s.max_y;

            let same_height = v.position.z == pos.z;
            let x_ends_on_the_start = seq_max_x == min_x;
            let start_y_within_borders = min_y >= seq_min_y && min_y <= seq_max_y;
            let end_y_within_borders = max_y >= seq_min_y && max_y <= seq_max_y;

            same_height
                && (start_y_within_borders || end_y_within_borders)
                && x_ends_on_the_start
        })
        .flat_map(|(s, v)| {
            let seq_start_y = v.position.y as usize;
            let seq_end_y = (v.position.y + s.max_y) as usize;

            seq_start_y..=seq_end_y
        })
        .collect();

    !(min_y as usize..=max_y as usize).into_iter().all(|y| adjoining_plane_y.contains(&y))
}

fn is_right_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;
    let max_x = pos.x + shape.max_x;

    let adjoining_plane_y: Vec<usize> = all_shapes.iter()
        .filter(|(s, v)| {
            let seq_min_x = v.position.x;
            let seq_min_y = v.position.y;
            let seq_max_y = v.position.y + s.max_y;

            let same_height = v.position.z == pos.z;
            let x_starts_on_the_end = seq_min_x == max_x;
            let start_y_within_borders = min_y >= seq_min_y && min_y <= seq_max_y;
            let end_y_within_borders = max_y >= seq_min_y && max_y <= seq_max_y;

            same_height
                && (start_y_within_borders || end_y_within_borders)
                && x_starts_on_the_end
        })
        .flat_map(|(s, v)| {
            let seq_start_y = v.position.y as usize;
            let seq_end_y = (v.position.y + s.max_y) as usize;

            seq_start_y..=seq_end_y
        })
        .collect();

    !(min_y as usize..=max_y as usize).into_iter().all(|y| adjoining_plane_y.contains(&y))
}

fn is_back_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    let min_x = pos.x as usize;
    let max_x = (pos.x + shape.max_x) as usize;

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|(s, v)| {
            let seq_end_y = v.position.y + s.max_y;
            let same_height = v.position.z == pos.z;
            let ends_on_the_start = seq_end_y == pos.y;

            same_height && ends_on_the_start
        })
        .flat_map(|(s, v)| {
            let seq_start_x = v.position.x as usize;
            let seq_end_x = (v.position.x + s.max_x) as usize;

            seq_start_x..=seq_end_x
        })
        .collect();

    !(min_x..=max_x).into_iter().all(|x| adjoining_plane_x.contains(&x))
}

fn is_forward_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    let min_x = pos.x as usize;
    let max_x = (pos.x + shape.max_x) as usize;
    let max_y = pos.y + shape.max_y;

    let adjoining_plane_x: Vec<usize> = all_shapes.iter()
        .filter(|(_, v)| {
            let seq_min_y = v.position.y;
            let same_height = v.position.z == pos.z;
            let starts_on_the_end = seq_min_y == max_y;

            same_height && starts_on_the_end
        })
        .flat_map(|(s, v)| {
            let seq_start_x = v.position.x as usize;
            let seq_end_x = (v.position.x + s.max_x) as usize;

            seq_start_x..=seq_end_x
        })
        .collect();

    !(min_x..=max_x).into_iter().all(|x| adjoining_plane_x.contains(&x))
}

fn is_bottom_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    if pos.z == 0.0 {
        return false;
    }

    let min_x = pos.x;
    let max_x = pos.x + shape.max_x;
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;

    let next_z_layer = get_next_z_layer(pos, shape, all_shapes, -1.0);

    if next_z_layer.is_empty() {
        return true;
    }

    for y in min_y as usize..max_y as usize {
        for x in min_x as usize..max_x as usize {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

// TODO fix random bug of rendering
fn is_top_side_needed(pos: &Point, shape: &shape::Box, all_shapes: &[(shape::Box, &Voxel)]) -> bool {
    let min_x = pos.x;
    let max_x = pos.x + shape.max_x;
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;

    let next_z_layer = get_next_z_layer(pos, shape, all_shapes, 1.0);

    if next_z_layer.is_empty() {
        return true;
    }

    for y in min_y as usize..max_y as usize {
        for x in min_x as usize..max_x as usize {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

fn get_next_z_layer<'a>(pos: &'a Point, shape: &'a shape::Box, all_shapes: &'a [(shape::Box, &Voxel)], z_bonus: f32) -> Vec<(usize, usize)> {
    let min_x = pos.x;
    let max_x = pos.x + shape.max_x;
    let min_y = pos.y;
    let max_y = pos.y + shape.max_y;

    all_shapes.iter()
        .filter(|(s, v)| {
            let seq_min_x = v.position.x;
            let seq_max_x = v.position.x + s.max_x;
            let seq_min_y = v.position.y;
            let seq_max_y = v.position.y + s.max_y;

            let has_needed_height = v.position.z == pos.z + z_bonus;
            let start_y_within_borders = min_y >= seq_min_y && min_y <= seq_max_y;
            let end_y_within_borders = max_y >= seq_min_y && max_y <= seq_max_y;
            let start_x_within_borders = min_x >= seq_min_x && min_x <= seq_max_x;
            let end_x_within_borders = max_x >= seq_min_x && max_x <= seq_max_x;

            has_needed_height
                && (start_y_within_borders || end_y_within_borders)
                && (start_x_within_borders || end_x_within_borders)
        })
        .flat_map(|(s, v)| {
            let seq_min_x = v.position.x as usize;
            let seq_max_x = (v.position.x + s.max_x) as usize;
            let seq_min_y = v.position.y as usize;
            let seq_max_y = (v.position.y + s.max_y) as usize;

            let mut coordinates = vec![];
            for y in seq_min_y..seq_max_y {
                for x in seq_min_x..seq_max_x {
                    coordinates.push((x, y));
                }
            }

            coordinates
        })
        .collect()
}

