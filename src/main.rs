use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};

use crate::entity::voxel::VoxelMaterial;
use crate::level::read_level;
use crate::system::light::spawn_light_source;

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
        // .add_startup_system(system::light::setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let map = read_level("debug");

    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    const TRANSPARENT: Color = Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 };
    let grass_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/grass.png")),
        ..Default::default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/stone.png")),
        ..Default::default()
    });
    let dirt_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/dirt.png")),
        ..Default::default()
    });
    let bedrock_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/bedrock.png")),
        ..Default::default()
    });
    let wooden_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/wooden_planks.png")),
        ..Default::default()
    });
    let light_material = materials.add(StandardMaterial {
        base_color: TRANSPARENT,
        base_color_texture: Some(asset_server.load("texture/block/light.png")),
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
            VoxelMaterial::WoodenPlanks => { wooden_material.clone() }
            VoxelMaterial::Light => { light_material.clone() }
            VoxelMaterial::Unknown => { unknown_material.clone() }
        };
        commands.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });

        if voxel.material == VoxelMaterial::Light {
            spawn_light_source(pos.x, pos.y, pos.z, &mut commands);
        }
    }
}