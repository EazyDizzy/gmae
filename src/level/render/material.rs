use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::Uuid;
use convert_case::{Case, Casing};
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
    const TEXTURE_PATH: &str = "./assets/texture/block/";
    let material_name = get_material_file_name(voxel_material);
    let material_id = format!("{TEXTURE_PATH}{material_name}");

    image::open(material_id).unwrap()
}

fn get_material_file_name(voxel_material: Material) -> String {
    format!("{voxel_material:?}.png").to_case(Case::Snake)
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
    let generate_path = |material: Material| -> String{
        format!("texture/block/{}", get_material_file_name(material))
    };

    let _id = materials.set(
        generate_material_handle_id(Material::Grass),
        create_default_material(asset_server.load(&generate_path(Material::Grass))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Stone),
        create_default_material(asset_server.load(&generate_path(Material::Stone))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Dirt),
        create_default_material(asset_server.load(&generate_path(Material::Dirt))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Bedrock),
        create_default_material(asset_server.load(&generate_path(Material::Bedrock))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::WoodenPlanks),
        create_default_material(asset_server.load(&generate_path(Material::WoodenPlanks))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::OrangeLight), StandardMaterial {
            base_color_texture: Some(asset_server.load(&generate_path(Material::OrangeLight))),
            reflectance: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..Default::default()
        });
    let _id = materials.set(generate_material_handle_id(Material::BlueLight), StandardMaterial {
        base_color_texture: Some(asset_server.load(&generate_path(Material::BlueLight))),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(
        generate_material_handle_id(Material::DirtPath),
        create_default_material(asset_server.load(&generate_path(Material::DirtPath))),
    );
    let _id = materials.set(generate_material_handle_id(Material::Glass), StandardMaterial {
        base_color_texture: Some(asset_server.load(&generate_path(Material::Glass))),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });
    let _id = materials.set(
        generate_material_handle_id(Material::Hay),
        create_default_material(asset_server.load(&generate_path(Material::Hay))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Pumpkin),
        create_default_material(asset_server.load(&generate_path(Material::Pumpkin))),
    );
    let _id = materials.set(generate_material_handle_id(Material::Unknown), StandardMaterial {
        base_color: Color::PINK,
        ..Default::default()
    });
    let _id = materials.set(
        generate_material_handle_id(Material::Cobblestone),
        create_default_material(asset_server.load(&generate_path(Material::Cobblestone))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::MossyCobblestone),
        create_default_material(asset_server.load(&generate_path(Material::MossyCobblestone))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::OakLeaves),
        StandardMaterial {
            base_color: Color::DARK_GREEN,
            base_color_texture: Some(asset_server.load(&generate_path(Material::OakLeaves))),
            reflectance: 0.0,
            ..Default::default()
        }
    );
    let _id = materials.set(
        generate_material_handle_id(Material::OakLog),
        create_default_material(asset_server.load(&generate_path(Material::OakLog))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::WhiteTerracotta),
        create_default_material(asset_server.load(&generate_path(Material::WhiteTerracotta))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Farmland),
        create_default_material(asset_server.load(&generate_path(Material::Farmland))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::StrippedOakLog),
        create_default_material(asset_server.load(&generate_path(Material::StrippedOakLog))),
    );
    let _id = materials.set(
        generate_material_handle_id(Material::Water),
        create_default_material(asset_server.load(&generate_path(Material::Water))),
    );
}

fn create_default_material(image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        reflectance: 0.0,
        perceptual_roughness: 0.0,
        metallic: 0.0,
        ..Default::default()
    }
}