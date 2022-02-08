use std::collections::HashMap;
use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::entity::voxel::Voxel;
use crate::Material;

#[derive(Debug)]
pub struct VoxelSequence<'a> {
    pub start: &'a Voxel,
    end: &'a Voxel,
}

impl<'a> VoxelSequence<'a> {
    pub fn is_not_transparent(&self) -> bool {
        self.start.material != Material::Glass
    }

    pub fn contains_y(&self, y: f32) -> bool {
        let (start_y, end_y) = self.y_borders();

        y >= start_y && y <= end_y
    }
    pub fn contains_x(&self, x: f32) -> bool {
        let (start_x, end_x) = self.x_borders();

        x >= start_x && x <= end_x
    }

    pub fn has_x_start_on(&self, x: f32) -> bool {
        self.start.position.x == x
    }
    pub fn has_x_end_on(&self, x: f32) -> bool {
        let (.., end_x) = self.x_borders();

        end_x == x
    }
    pub fn has_y_start_on(&self, y: f32) -> bool {
        self.start.position.y == y
    }
    pub fn has_y_end_on(&self, y: f32) -> bool {
        let (.., end_y) = self.y_borders();

        end_y == y
    }

    pub fn has_height(&self, z: f32) -> bool {
        self.start.position.z == z
    }

    pub fn covered_coordinates(&self) -> Vec<(usize, usize)> {
        let (start_x, end_x) = self.x_borders();
        let (start_y, end_y) = self.y_borders();

        let mut coordinates = vec![];
        // TODO use covered_x/y
        for y in start_y as usize..end_y as usize {
            for x in start_x as usize..end_x as usize {
                coordinates.push((x, y));
            }
        }

        coordinates
    }
    pub fn covered_x(&self) -> RangeInclusive<usize> {
        let (start_x, end_x) = self.x_borders();

        start_x as usize..=end_x as usize
    }
    pub fn covered_y(&self) -> RangeInclusive<usize> {
        let (start_y, end_y) = self.y_borders();

        start_y as usize..=end_y as usize
    }

    fn height(&self) -> u32 {
        (self.end.position.y + 1.0 - self.start.position.y) as u32
    }

    pub fn get_box(&self) -> shape::Box {
        shape::Box {
            min_x: 0.0,
            max_x: self.x_width(),
            min_y: 0.0,
            max_y: self.y_height(),
            min_z: 0.0,
            max_z: self.end.position.z - self.start.position.z + 1.0,
        }
    }

    fn x_width(&self) -> f32 {
        self.end.position.x - self.start.position.x + 1.0
    }
    fn y_height(&self) -> f32 {
        self.end.position.y - self.start.position.y + 1.0
    }

    fn x_borders(&self) -> (f32, f32) {
        let start_x = self.start.position.x;
        let end_x = self.start.position.x + self.x_width();

        (start_x, end_x)
    }
    fn y_borders(&self) -> (f32, f32) {
        let start_y = self.start.position.y;
        let end_y = self.start.position.y + self.y_height();

        (start_y, end_y)
    }
}

pub fn merge_voxels(voxels: &[Voxel], max_voxels_per_dimension: u32) -> Vec<VoxelSequence> {
    let grouped_voxels = group_voxels_by_coordinates(voxels);

    let mut all_sequences = vec![];

    for (_, plate) in grouped_voxels {
        let mut plane_sequences = vec![];

        for (y, row) in &plate {
            let row_sequences = merge_voxels_row(row.clone(), max_voxels_per_dimension);

            plane_sequences = stretch_sequences_by_y(row_sequences, plane_sequences, *y, max_voxels_per_dimension);
        }

        all_sequences.extend(plane_sequences);
    }

    all_sequences
}

fn stretch_sequences_by_y<'a>(
    row_sequences: Vec<VoxelSequence<'a>>,
    mut plane_sequences: Vec<VoxelSequence<'a>>,
    y: usize,
    max_voxels_per_dimension: u32,
) -> Vec<VoxelSequence<'a>> {
    let mut sequences_to_append = vec![];
    let mut prev_row_sequences: Vec<&mut VoxelSequence> = plane_sequences.iter_mut()
        .filter(|s: &&mut VoxelSequence| {
            s.end.position.y == y as f32 - 1.0
        }).collect();

    for sequence in row_sequences {
        let same_sequence = prev_row_sequences.iter_mut().find(|s| {
            s.start.position.x == sequence.start.position.x
                && s.end.position.x == sequence.end.position.x
                && s.start.material == sequence.start.material
                && should_merge(sequence.start.material)
        });

        if let Some(same) = same_sequence {
            if same.height() + sequence.height() < max_voxels_per_dimension {
                same.end = sequence.end;
            } else {
                sequences_to_append.push(sequence);
            }
        } else {
            sequences_to_append.push(sequence);
        }
    }

    plane_sequences.append(&mut sequences_to_append);

    plane_sequences
}

fn merge_voxels_row(mut row: Vec<&Voxel>, max_voxels_per_dimension: u32) -> Vec<VoxelSequence> {
    row.sort_by(|a, b| {
        a.position.x.partial_cmp(&b.position.x).unwrap()
    });

    let mut x_sequences = vec![];
    let mut start_voxel = row[0];
    let mut prev_voxel = row[0];

    for voxel in row.into_iter().skip(1) {
        let concatenation_width = (prev_voxel.position.x - start_voxel.position.x) as u32;
        let stop_concatenation = voxel.position.x != prev_voxel.position.x + 1.0
            || voxel.material != prev_voxel.material
            || !should_merge(prev_voxel.material)
            || concatenation_width + 1 == max_voxels_per_dimension;

        if stop_concatenation {
            x_sequences.push(
                VoxelSequence {
                    start: start_voxel,
                    end: prev_voxel,
                });

            start_voxel = voxel;
        }

        prev_voxel = voxel;
    }
    x_sequences.push(
        VoxelSequence {
            start: start_voxel,
            end: prev_voxel,
        });

    x_sequences
}

fn group_voxels_by_coordinates(voxels: &[Voxel]) -> HashMap<usize, HashMap<usize, Vec<&Voxel>>> {
    let mut grouping = HashMap::new();

    for voxel in voxels {
        let z = voxel.position.z.round() as usize;
        let z_plane = grouping.entry(z).or_insert_with(HashMap::new);

        let y = voxel.position.y.round() as usize;
        let y_row = z_plane.entry(y).or_insert_with(Vec::new);

        y_row.push(voxel);
    }

    grouping
}

fn should_merge(material: Material) -> bool {
    ![
        Material::BlueLight,
        Material::OrangeLight
    ].contains(&material)
}