use std::ptr;

use crate::level::render::voxel_sequence::VoxelSequence;

pub mod cube;
pub mod triangle_prism;

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

#[allow(unused)]
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

#[allow(unused)]
fn is_bottom_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    if main_sequence.has_height(0.0) {
        return false;
    }

    let height = main_sequence.start_position().z - 1.0;
    let next_z_layer = get_next_z_layer(main_sequence, sequences, height);

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
    let height = main_sequence.end_position().z + 1.0;
    let next_z_layer = get_next_z_layer(main_sequence, sequences, height);

    for y in main_sequence.covered_y() {
        for x in main_sequence.covered_x() {
            if !next_z_layer.contains(&(x, y)) {
                return true;
            }
        }
    }

    false
}

fn get_next_z_layer<'a>(main_sequence: &'a VoxelSequence, all_sequences: &'a [VoxelSequence], height: f32) -> Vec<(usize, usize)> {
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