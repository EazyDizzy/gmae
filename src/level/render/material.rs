use std::time::Instant;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::Uuid;
use image::{DynamicImage, GenericImageView, Pixel, Rgba};

use crate::Material;

pub const TEXTURE_SIZE: u32 = 64;

const COLOR_SIZE: u32 = Rgba::<u8>::CHANNEL_COUNT as u32;
const BYTES_IN_ROW: u32 = TEXTURE_SIZE * COLOR_SIZE;

// TODO dynamically select texture size based on wgpu limits
pub fn merge_materials(
    material: Material,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    number_of_images_wide: u32,
    number_of_images_in_height: u32,
) -> Handle<StandardMaterial> {
    if number_of_images_wide == 1 && number_of_images_in_height == 1 {
        return materials.get_handle(generate_material_handle_id(material));
    }
    let handle_id = generate_dynamic_material_handle_id(material, number_of_images_wide, number_of_images_in_height);

    if materials.get(handle_id).is_some() {
        return materials.get_handle(handle_id);
    }

    let start = Instant::now();
    let new_texture_width = TEXTURE_SIZE * number_of_images_wide;
    let new_texture_height = TEXTURE_SIZE * number_of_images_in_height;
    let original_image_pixels: Vec<u8> = get_basic_image_for_material(material)
        .pixels()
        .flat_map(|(.., p)| p.0)
        .collect();

    let mut pixel_row = Vec::with_capacity((BYTES_IN_ROW * number_of_images_wide) as usize);
    for y in 0..TEXTURE_SIZE {
        let start = y * BYTES_IN_ROW;
        let end = start + BYTES_IN_ROW;
        let pixels_slice = &original_image_pixels[start as usize..end as usize];

        for _ in 0..number_of_images_wide {
            pixel_row.extend(pixels_slice);
        }
    }

    let mut pixel_buf = Vec::with_capacity((COLOR_SIZE * new_texture_width * new_texture_height) as usize);
    for _ in 0..number_of_images_in_height {
        pixel_buf.extend(&pixel_row);
    }

    if material == Material::Grass {
        println!("material: {:?} {} {} {:?}", material, number_of_images_wide, number_of_images_in_height, start.elapsed());
    }

    // raw creation to prevent triple conversion of image buffer
    let image = Image::new(
        Extent3d {
            width: new_texture_width,
            height: new_texture_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixel_buf,
        TextureFormat::Rgba8UnormSrgb,
    );

    let image_handle = images.add(image);
    let original_material = materials.get(generate_material_handle_id(material))
        .expect(&format!("Cannot get material for {:?}", material))
        .clone();

    materials.set(
        handle_id,
        StandardMaterial {
            base_color_texture: Some(image_handle),
            ..original_material
        },
    )
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

fn generate_dynamic_material_handle_id(voxel_material: Material, image_width: u32, image_height: u32) -> HandleId {
    let id = Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, voxel_material as u8, image_width as u8, image_height as u8]);

    HandleId::Id(id, 0)
}

fn generate_material_handle_id(voxel_material: Material) -> HandleId {
    generate_dynamic_material_handle_id(voxel_material, 0, 0)
}

#[allow(clippy::needless_pass_by_value)]
pub fn setup(mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    let _id = materials.set(generate_material_handle_id(Material::Grass), create_material(asset_server.load("texture/block/grass.png")));
    let _id = materials.set(generate_material_handle_id(Material::Stone), create_material(asset_server.load("texture/block/stone.png")));
    let _id = materials.set(generate_material_handle_id(Material::Dirt), create_material(asset_server.load("texture/block/dirt.png")));
    let _id = materials.set(generate_material_handle_id(Material::Bedrock), create_material(asset_server.load("texture/block/bedrock.png")));
    let _id = materials.set(generate_material_handle_id(Material::WoodenPlanks), create_material(asset_server.load("texture/block/wooden_planks.png")));
    let _id = materials.set(generate_material_handle_id(Material::OrangeLight), StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/orange_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(generate_material_handle_id(Material::BlueLight), StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/blue_light.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(generate_material_handle_id(Material::DirtPath), create_material(asset_server.load("texture/block/dirt_path.png")));
    let _id = materials.set(generate_material_handle_id(Material::Glass), StandardMaterial {
        base_color_texture: Some(asset_server.load("texture/block/glass.png")),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let _id = materials.set(generate_material_handle_id(Material::Hay), create_material(asset_server.load("texture/block/hay.png")));
    let _id = materials.set(generate_material_handle_id(Material::Pumpkin), create_material(asset_server.load("texture/block/pumpkin.png")));
    let _id = materials.set(generate_material_handle_id(Material::Unknown), StandardMaterial {
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