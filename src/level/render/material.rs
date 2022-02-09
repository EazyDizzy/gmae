use std::time::Instant;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::Uuid;
use image::{DynamicImage, GenericImageView, Pixel, Rgba};

use crate::Material;

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

pub const TEXTURE_SIZE: u32 = 64;

// TODO dynamically select texture size based on wgpu limits
pub fn merge_materials(
    voxel_material: Material,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    number_of_images_wide: u32,
    number_of_images_in_height: u32,
) -> Handle<StandardMaterial> {
    if number_of_images_wide == 1 && number_of_images_in_height == 1 {
        return materials.get_handle(get_material_id(voxel_material));
    }
    let handle_id = generate_handle_id_for_material(voxel_material, number_of_images_wide, number_of_images_in_height);

    if materials.get(handle_id).is_some() {
        return materials.get_handle(handle_id);
    }

    let basic_image = get_basic_image_for_material(voxel_material);

    let new_texture_width = TEXTURE_SIZE * number_of_images_wide;
    let new_texture_height = TEXTURE_SIZE * number_of_images_in_height;
    const COLOR_SIZE: u32 = Rgba::<u8>::CHANNEL_COUNT as u32;

    let mut image_pixels = Vec::with_capacity((COLOR_SIZE * TEXTURE_SIZE * TEXTURE_SIZE) as usize);

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let p = basic_image.get_pixel(x, y);
            image_pixels.extend(p.0);
        }
    }
    let mut pixel_row = Vec::with_capacity((4 * new_texture_width * new_texture_height) as usize);
    for y in 0..TEXTURE_SIZE {
        let bytes_in_row = TEXTURE_SIZE * COLOR_SIZE;
        let start = y * bytes_in_row ;
        let end = start + bytes_in_row;
        let pixels_slice = &image_pixels[start as usize..end as usize];

        for _ in 0..number_of_images_wide {
            pixel_row.extend(pixels_slice);
        }
    }
    let mut pixel_rows = Vec::with_capacity((4 * new_texture_width * new_texture_height) as usize);
    for _ in 0..number_of_images_in_height {
        pixel_rows.extend(&pixel_row);
    }

    let start = Instant::now();

    let img_buf: image::ImageBuffer<Rgba<u8>, _> =
        image::ImageBuffer::from_vec(new_texture_width, new_texture_height, pixel_rows)
            .expect("cannot create buffer from vector");

    if voxel_material == Material::Grass {
        println!("buffer creation {} {} {:?}", number_of_images_wide, number_of_images_in_height, start.elapsed());
    }

    // raw creation to prevent triple conversion of image (img_buf -> slice_buf -> DynamicImage -> img_buf)
    let image = Image::new(
        Extent3d {
            width: new_texture_width,
            height: new_texture_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        img_buf.into_raw(),
        TextureFormat::Rgba8UnormSrgb,
    );

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

fn generate_handle_id_for_material(voxel_material: Material, image_width: u32, image_height: u32) -> HandleId {
    let id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, voxel_material as u8, image_width as u8, image_height as u8]);

    HandleId::Id(id, 0)
}

fn get_basic_image_for_material(voxel_material: Material) -> DynamicImage {
    let material_id = match voxel_material {
        Material::Grass => "./assets/texture/block/grass.png",
        Material::Stone => "./assets/texture/block/stone.png",
        Material::Dirt => "./assets/texture/block/dirt.png",
        Material::Bedrock => "./assets/texture/block/bedrock.png",
        Material::WoodenPlanks => "./assets/texture/block/wooden_planks.png",
        Material::OrangeLight => "./assets/texture/block/orange_light.png",
        Material::BlueLight => "./assets/texture/block/blue_light.png",
        Material::DirtPath => "./assets/texture/block/dirt_path.png",
        Material::Glass => "./assets/texture/block/glass.png",
        Material::Hay => "./assets/texture/block/hay.png",
        Material::Pumpkin => "./assets/texture/block/pumpkin.png",
        Material::Unknown => "./assets/texture/block/unknown.png",
    };

    image::open(material_id).unwrap()
}

fn get_material_id(voxel_material: Material) -> HandleId {
    match voxel_material {
        Material::Grass => GRASS_MATERIAL_ID,
        Material::Stone => STONE_MATERIAL_ID,
        Material::Dirt => DIRT_MATERIAL_ID,
        Material::Bedrock => BEDROCK_MATERIAL_ID,
        Material::WoodenPlanks => WOODEN_PLANKS_MATERIAL_ID,
        Material::OrangeLight => ORANGE_LIGHT_MATERIAL_ID,
        Material::BlueLight => BLUE_LIGHT_MATERIAL_ID,
        Material::DirtPath => DIRT_PATH_MATERIAL_ID,
        Material::Glass => GLASS_MATERIAL_ID,
        Material::Hay => HAY_MATERIAL_ID,
        Material::Pumpkin => PUMPKIN_MATERIAL_ID,
        Material::Unknown => UNKNOWN_MATERIAL_ID,
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn setup(mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    let _id = materials.set(GRASS_MATERIAL_ID, create_material(asset_server.load("texture/block/grass.png")));
    let _id = materials.set(STONE_MATERIAL_ID, create_material(asset_server.load("texture/block/stone.png")));
    let _id = materials.set(DIRT_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt.png")));
    let _id = materials.set(BEDROCK_MATERIAL_ID, create_material(asset_server.load("texture/block/bedrock.png")));
    let _id = materials.set(WOODEN_PLANKS_MATERIAL_ID, create_material(asset_server.load("texture/block/wooden_planks.png")));
    let _id = materials.set(ORANGE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/orange_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(BLUE_LIGHT_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/blue_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(DIRT_PATH_MATERIAL_ID, create_material(asset_server.load("texture/block/dirt_path.png")));
    let _id = materials.set(GLASS_MATERIAL_ID, StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/glass.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let _id = materials.set(HAY_MATERIAL_ID, create_material(asset_server.load("texture/block/hay.png")));
    let _id = materials.set(PUMPKIN_MATERIAL_ID, create_material(asset_server.load("texture/block/pumpkin.png")));
    let _id = materials.set(UNKNOWN_MATERIAL_ID, StandardMaterial {
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