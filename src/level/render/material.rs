use std::collections::HashSet;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::texture::ImageType;
use bevy::utils::Uuid;
use image::{codecs, ColorType, DynamicImage, GenericImageView, ImageEncoder, Rgba};
use rand::{Rng, XorShiftRng};

use crate::level::util::get_rng;
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

pub fn concatenate_material(
    voxel_material: VoxelMaterial,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    image_width: u32,
    image_height: u32,
) -> Handle<StandardMaterial> {
    if image_width == 1 && image_height == 1 {
        return materials.get_handle(get_material_id(voxel_material));
    }
    let handle_id = generate_handle_id_for_material(voxel_material, image_width, image_height);

    if materials.get(handle_id).is_some() {
        return materials.get_handle(handle_id);
    }

    let basic_image = get_basic_image_for_material(voxel_material);

    let texture_width = basic_image.width() * image_width;
    let texture_height = basic_image.width() * image_height;

    let mut img_buf = image::ImageBuffer::new(
        texture_width,
        texture_height,
    );

    for w in 0..image_width {
        let x_bonus = w * basic_image.width();

        for h in 0..image_height {
            let y_bonus = h * basic_image.height();

            basic_image.pixels()
                .for_each(|(x, y, pixel)| {
                    img_buf.put_pixel(x + x_bonus, y + y_bonus, pixel);
                });
        }
    }

    let mut out = vec![];
    codecs::png::PngEncoder::new(&mut out).write_image(
        img_buf.as_raw(),
        texture_width,
        texture_height,
        ColorType::Rgba8,
    )
        .expect("Cannot write to memory");

    let image = Image::from_buffer(&out, ImageType::MimeType("image/png"))
        .expect("Failed to convert into image");

    let image_handle = images.add(image);
    let original_material = materials.get(get_material_id(voxel_material))
        .expect(&format!("Cannot get material for {:?}", voxel_material))
        .clone();

    materials.set(
        handle_id,
        StandardMaterial {
            base_color_texture: Some(image_handle),
            ..original_material
        },
    )
}

fn generate_handle_id_for_material(voxel_material: VoxelMaterial, image_width: u32, image_height: u32) -> HandleId {
    let id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, voxel_material as u8, image_width as u8, image_height as u8]);

    HandleId::Id(id, 0)
}

fn get_basic_image_for_material(voxel_material: VoxelMaterial) -> DynamicImage {
    let material_id = match voxel_material {
        VoxelMaterial::Grass => "./assets/texture/block/grass.png",
        VoxelMaterial::Stone => "./assets/texture/block/stone.png",
        VoxelMaterial::Dirt => "./assets/texture/block/dirt.png",
        VoxelMaterial::Bedrock => "./assets/texture/block/bedrock.png",
        VoxelMaterial::WoodenPlanks => "./assets/texture/block/wooden_planks.png",
        VoxelMaterial::OrangeLight => "./assets/texture/block/orange_light.png",
        VoxelMaterial::BlueLight => "./assets/texture/block/blue_light.png",
        VoxelMaterial::DirtPath => "./assets/texture/block/dirt_path.png",
        VoxelMaterial::Glass => "./assets/texture/block/glass.png",
        VoxelMaterial::Hay => "./assets/texture/block/hay.png",
        VoxelMaterial::Pumpkin => "./assets/texture/block/pumpkin.png",
        // TODO
        VoxelMaterial::Unknown => "./assets/texture/block/grass_0.png",
    };

    image::open(material_id).unwrap()
}

fn get_material_id(voxel_material: VoxelMaterial) -> HandleId {
    match voxel_material {
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
    }
}

pub fn setup(mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    let _ = materials.set(GRASS_MATERIAL_ID, create_material(asset_server.load("texture/block/grass.png")));
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