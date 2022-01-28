use std::collections::HashSet;
use std::io::{BufWriter};
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::texture::ImageType;
use bevy::utils::Uuid;
use image::{codecs, ColorType, DynamicImage, GenericImageView, ImageEncoder, Rgba};
use rand::{Rng, XorShiftRng};

use crate::VoxelMaterial;

const MATERIAL_UUID: Uuid = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const GRASS_MATERIAL_ID: HandleId = HandleId::Id(MATERIAL_UUID, 1);
const GRASS_MATERIAL2_ID: HandleId = HandleId::Id(MATERIAL_UUID, 10002);
const GRASS_MATERIAL3_ID: HandleId = HandleId::Id(MATERIAL_UUID, 10003);
const GRASS_MATERIAL4_ID: HandleId = HandleId::Id(MATERIAL_UUID, 10004);

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

pub fn get_material(voxel_material: VoxelMaterial, materials: &Res<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    if voxel_material == VoxelMaterial::Grass {
        let mut rng: XorShiftRng = {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_micros();
            rand::SeedableRng::from_seed([now, now, now, now])
        };
        let material_id = *rng.choose(&[GRASS_MATERIAL_ID, GRASS_MATERIAL2_ID, GRASS_MATERIAL3_ID, GRASS_MATERIAL4_ID]).unwrap();

        return materials.get_handle(material_id);
    }

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

pub fn setup(mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    let img = image::open("./assets/texture/block/grass_0.png").unwrap();
    let grass_handle = images.set(GRASS_MATERIAL_ID, create_random_texture_from(&img));
    let grass1_handle = images.set(GRASS_MATERIAL2_ID, create_random_texture_from(&img));
    let grass2_handle = images.set(GRASS_MATERIAL3_ID, create_random_texture_from(&img));
    let grass3_handle = images.set(GRASS_MATERIAL4_ID, create_random_texture_from(&img));

    let _ = materials.set(GRASS_MATERIAL_ID, create_material(grass_handle));
    let _ = materials.set(GRASS_MATERIAL2_ID, create_material(grass1_handle));
    let _ = materials.set(GRASS_MATERIAL3_ID, create_material(grass2_handle));
    let _ = materials.set(GRASS_MATERIAL4_ID, create_material(grass3_handle));
    let _ = materials.set(STONE_MATERIAL_ID, create_material(asset_server.load("texture/block/stone.png")));
    let _ = materials.set(DIRT_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt.png")));
    let _ = materials.set(BEDROCK_MATERIAL_ID, create_material(asset_server.load("texture/block/bedrock.png")));
    let _ = materials.set(WOODEN_PLANKS_MATERIAL_ID, create_material(asset_server.load("texture/block/wooden_planks.png")));
    let _ = materials.set(ORANGE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/orange_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _ = materials.set(BLUE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/blue_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _ = materials.set(DIRT_PATH_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt_path.png")));
    let _ = materials.set(GLASS_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/glass.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let _ = materials.set(HAY_MATERIAL_ID, create_material(asset_server.load("texture/block/hay.png")));
    let _ = materials.set(PUMPKIN_MATERIAL_ID, create_material(asset_server.load("texture/block/pumpkin.png")));
    let _ = materials.set(UNKNOWN_MATERIAL_ID, StandardMaterial {
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

fn create_random_texture_from(texture: &DynamicImage) -> Image {
    let mut stats: HashSet<Rgba<u8>> = HashSet::default();
    for (_, _, pixel) in texture.pixels() {
        stats.insert(pixel);
    }
    let unique_pixels: Vec<Rgba<u8>> = stats.into_iter().collect();

    let mut rng: XorShiftRng = {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_micros();
        rand::SeedableRng::from_seed([now, now, now, now])
    };

    let mut img_buf = image::ImageBuffer::new(texture.height(), texture.width());
    for (_, _, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = *rng.choose(unique_pixels.as_slice()).unwrap();
    }

    let mut out = &mut BufWriter::new(Vec::new());
    codecs::png::PngEncoder::new(&mut out).write_image(
        img_buf.as_raw(),
        texture.width(),
        texture.height(),
        ColorType::Rgba8,
    )
        .expect("Cannot write to memory");

    Image::from_buffer(out.buffer(), ImageType::Extension("png"))
        .expect("Failed to convert into image")
}