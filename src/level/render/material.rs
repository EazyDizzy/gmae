use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::utils::Uuid;

use crate::VoxelMaterial;

const MATERIAL_UUID: Uuid = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const GRASS_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 1);
const STONE_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 2);
const DIRT_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 3);
const BEDROCK_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 4);
const WOODEN_PLANKS_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 5);
const ORANGE_LIGHT_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 6);
const BLUE_LIGHT_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 7);
const DIRT_PATH_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 8);
const GLASS_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 9);
const HAY_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 10);
const PUMPKIN_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 11);
const UNKNOWN_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 666);

pub fn get_material(voxel_material: VoxelMaterial, materials: &ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    let material_id = match voxel_material {
        VoxelMaterial::Grass => GRASS_MATERIAL_ID,
        VoxelMaterial::Stone => STONE_MATERIAL_ID,
        VoxelMaterial::Dirt => DIRT_MATERIAL_ID,
        VoxelMaterial::Bedrock => BEDROCK_MATERIAL_ID,
        VoxelMaterial::WoodenPlanks => WOODEN_PLANKS_MATERIAL_ID,
        VoxelMaterial::OrangeLight => ORANGE_LIGHT_MATERIAL_ID,
        VoxelMaterial::BlueLight => BLUE_LIGHT_MATERIAL_ID,
        VoxelMaterial::DirtPath => DIRT_PATH_MATERIAL_ID,
        VoxelMaterial::Glass => GLASS_MATERIAL_ID,
        VoxelMaterial::Hay => HAY_MATERIAL_ID,
        VoxelMaterial::Pumpkin => PUMPKIN_MATERIAL_ID,
        VoxelMaterial::Unknown => UNKNOWN_MATERIAL_ID,
    };

    materials.get_handle(material_id)
}

pub fn load_materials(materials: &mut ResMut<Assets<StandardMaterial>>, asset_server: &Res<AssetServer>) {
    materials.set(GRASS_MATERIAL_ID, create_material(asset_server.load("texture/block/grass.png")));
    materials.set(STONE_MATERIAL_ID, create_material(asset_server.load("texture/block/stone.png")));
    materials.set(DIRT_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt.png")));
    materials.set(BEDROCK_MATERIAL_ID, create_material(asset_server.load("texture/block/bedrock.png")));
    materials.set(WOODEN_PLANKS_MATERIAL_ID, create_material(asset_server.load("texture/block/wooden_planks.png")));
    materials.set(ORANGE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/orange_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    materials.set(BLUE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/blue_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    materials.set(DIRT_PATH_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt_path.png")));
    materials.set(GLASS_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/glass.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    materials.set(HAY_MATERIAL_ID, create_material(asset_server.load("texture/block/hay.png")));
    materials.set(PUMPKIN_MATERIAL_ID, create_material(asset_server.load("texture/block/pumpkin.png")));
    materials.set(UNKNOWN_MATERIAL_ID, StandardMaterial {
        base_color: Color::PINK,
        ..Default::default()
    });
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