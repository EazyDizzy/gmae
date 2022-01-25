use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use crate::entity::voxel::VoxelMaterial;
use crate::level::read_level;

mod system;
mod level;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(system::camera::setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let map = read_level("debug");
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let grass_material = materials.add(StandardMaterial {
        base_color: Color::YELLOW_GREEN,
        ..Default::default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..Default::default()
    });
    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::ORANGE_RED,
        ..Default::default()
    });
    let bedrock_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        ..Default::default()
    });
    let unknown_material = materials.add(StandardMaterial {
        base_color: Color::PINK,
        ..Default::default()
    });

    for voxel in map {
        let pos = voxel.position;
        let material = match voxel.material {
            VoxelMaterial::Bedrock => { bedrock_material.clone() }
            VoxelMaterial::Stone => { stone_material.clone() }
            VoxelMaterial::Grass => { grass_material.clone() }
            VoxelMaterial::Dirt => { dirt_material.clone() }
            VoxelMaterial::Unknown => { unknown_material.clone() }
        };
        commands.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material,
            transform: Transform::from_xyz(pos.x as f32 * 1.05, pos.y as f32 * 1.05, pos.z as f32 * 1.05),
            ..Default::default()
        });
    }
}