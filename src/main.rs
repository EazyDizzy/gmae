use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_fly_camera::FlyCameraPlugin;

use crate::entity::voxel::VoxelMaterial;
use crate::level::read_level;
use crate::system::camera::{cursor_grab, initial_grab_cursor};
use crate::system::light::{spawn_blue_light_source, spawn_orange_light_source};

mod system;
mod level;
mod entity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FlyCameraPlugin)
        .add_startup_system(setup)
        .add_startup_system(system::camera::setup.system())
        .add_startup_system(initial_grab_cursor)
        .add_system(cursor_grab)
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
    let grass_material = materials.add(create_material(asset_server.load("texture/block/grass.png")));
    let stone_material = materials.add(create_material(asset_server.load("texture/block/stone.png")));
    let dirt_material = materials.add(create_material(asset_server.load("texture/block/dirt.png")));
    let bedrock_material = materials.add(create_material(asset_server.load("texture/block/bedrock.png")));
    let wooden_material = materials.add(create_material(asset_server.load("texture/block/wooden_planks.png")));
    let orange_light_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/orange_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let blue_light_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/blue_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let dirt_path_material = materials.add(create_material(asset_server.load("texture/block/dirt_path.png")));
    let glass_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/glass.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let hay_material = materials.add(create_material(asset_server.load("texture/block/hay.png")));
    let pumpkin_material = materials.add(create_material(asset_server.load("texture/block/pumpkin.png")));
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
            VoxelMaterial::OrangeLight => { orange_light_material.clone() }
            VoxelMaterial::BlueLight => { blue_light_material.clone() }
            VoxelMaterial::DirtPath => { dirt_path_material.clone() }
            VoxelMaterial::Glass => { glass_material.clone() }
            VoxelMaterial::Hay => { hay_material.clone() }
            VoxelMaterial::Pumpkin => { pumpkin_material.clone() }
            VoxelMaterial::Unknown => { unknown_material.clone() }
        };

        let mut entity_commands = commands.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material,
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            ..Default::default()
        });

        if voxel.material == VoxelMaterial::OrangeLight {
            spawn_orange_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
        if voxel.material == VoxelMaterial::BlueLight {
            spawn_blue_light_source(pos.x, pos.y, pos.z, &mut entity_commands);
        }
    }
}

fn create_material(image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        reflectance: 0.0,
        perceptual_roughness: 0.0,
        metallic: 0.0,
        ..Default::default()
    }
}