use std::ptr;

use crate::level::render::voxel_sequence::VoxelSequence;

pub mod cube;
pub mod triangle_prism;

fn is_left_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (start_x, ..) = main_sequence.x_borders();

    let adjoining_plane_z: Vec<usize> = all_shapes
        .iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.intersects_by_z(main_sequence)
                && sequence.has_x_end_on(start_x)
                && sequence.is_not_transparent()
                && !is_same_sequence(sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_z)
        .collect();

    !main_sequence
        .covered_z()
        .all(|z| adjoining_plane_z.contains(&z))
}

fn is_right_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_x) = main_sequence.x_borders();

    let adjoining_plane_z: Vec<usize> = all_shapes
        .iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.intersects_by_z(main_sequence)
                && sequence.has_x_start_on(end_x)
                && sequence.is_not_transparent()
                && !is_same_sequence(sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_z)
        .collect();

    !main_sequence
        .covered_z()
        .all(|z| adjoining_plane_z.contains(&z))
}

fn is_back_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (start_z, ..) = main_sequence.z_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes
        .iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.has_z_end_on(start_z + 1.0)
                && sequence.is_not_transparent()
                && !is_same_sequence(sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence
        .covered_x()
        .all(|x| adjoining_plane_x.contains(&x))
}

#[allow(unused)]
fn is_forward_side_visible(main_sequence: &VoxelSequence, all_shapes: &[VoxelSequence]) -> bool {
    let (.., end_z) = main_sequence.z_borders();

    let adjoining_plane_x: Vec<usize> = all_shapes
        .iter()
        .filter(|sequence| {
            sequence.has_same_height(main_sequence)
                && sequence.has_z_start_on(end_z)
                && sequence.is_not_transparent()
                && !is_same_sequence(sequence, main_sequence)
        })
        .flat_map(VoxelSequence::covered_x)
        .collect();

    !main_sequence
        .covered_x()
        .all(|x| adjoining_plane_x.contains(&x))
}

#[allow(unused)]
fn is_bottom_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    if main_sequence.has_height(0.0) {
        return false;
    }

    let height = main_sequence.start_position().y - 1.0;
    let next_y_layer = get_next_y_layer(main_sequence, sequences, height);

    for z in main_sequence.covered_z() {
        for x in main_sequence.covered_x() {
            if !next_y_layer.contains(&(x, z)) {
                return true;
            }
        }
    }

    false
}

fn is_top_side_visible(main_sequence: &VoxelSequence, sequences: &[VoxelSequence]) -> bool {
    let height = main_sequence.end_position().y + 1.0;
    let next_y_layer = get_next_y_layer(main_sequence, sequences, height);

    for z in main_sequence.covered_z() {
        for x in main_sequence.covered_x() {
            if !next_y_layer.contains(&(x, z)) {
                return true;
            }
        }
    }

    false
}

fn get_next_y_layer<'a>(
    main_sequence: &'a VoxelSequence,
    all_sequences: &'a [VoxelSequence],
    height: f32,
) -> Vec<(usize, usize)> {
    all_sequences
        .iter()
        .filter(|sequence| {
            sequence.has_height(height)
                && sequence.intersects_by_z(main_sequence)
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
