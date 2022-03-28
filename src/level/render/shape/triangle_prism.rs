use std::f32::consts::PI;

use bevy::prelude::*;
use lib::entity::point::Point;
use lib::entity::voxel::{Fastening, TrianglePrismProperties, WorldSide};

use crate::level::render::voxel_sequence::VoxelSequence;

pub fn create_triangle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sequence: &VoxelSequence,
    properties: &TrianglePrismProperties,
) {
    let mesh = asset_server.load("mesh/triangle.glb#Scene0");
    let position = sequence.start_position();
    commands.spawn_bundle((
        get_transform(properties, position),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(mesh);
    });
}

fn get_transform(properties: &TrianglePrismProperties, pos: &Point) -> Transform {
    let t = Transform::from_xyz(pos.x + 0.5, pos.y + 0.5, pos.z - 1.0);

    let rotation = match properties.fastening {
        Fastening::Top => {
            match properties.facing {
                WorldSide::North => Quat::from_euler(EulerRot::XYZ, 0.0, PI / 2.0, -PI / 2.0),
                WorldSide::South => Quat::from_euler(EulerRot::XYZ, 0.0, PI * 1.5, PI / 2.0),
                WorldSide::East => Quat::from_euler(EulerRot::XYZ, 0.0, PI, PI),
                WorldSide::West => Quat::from_euler(EulerRot::XYZ, 0.0, PI, 0.0),
            }
        }
        Fastening::Bottom => {
            match properties.facing {
                WorldSide::North => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -PI / 2.0),
                WorldSide::South => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, PI / 2.0),
                WorldSide::East => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, PI),
                WorldSide::West => Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0),
            }
        }
    };

    t.with_rotation(rotation)
}
