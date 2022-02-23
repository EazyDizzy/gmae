use std::collections::HashMap;

use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::utils::Uuid;
use convert_case::{Case, Casing};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use lib::entity::voxel::Voxel;
use lib::util::game_settings::GameSettings;
use rand::distributions::Distribution;
use rand::distributions::Uniform;

use crate::level::render::named_materials::{generate_name_for_voxels, NamedMaterials};
use crate::Material;

pub const TEXTURE_SIZE: u32 = 64;

const COLOR_SIZE: u32 = Rgba::<u8>::CHANNEL_COUNT as u32;
const BYTES_IN_ROW: u32 = TEXTURE_SIZE * COLOR_SIZE;

// TODO dynamically select texture based on graphic lvl settings
pub fn merge_materials(
    voxels: &Vec<Vec<&Voxel>>,
    named_materials: &mut ResMut<NamedMaterials>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    number_of_images_wide: u32,
    number_of_images_in_height: u32,
    settings: &Res<GameSettings>,
) -> Handle<StandardMaterial> {
    let material = voxels[0][0].material;
    if number_of_images_wide == 1 && number_of_images_in_height == 1 {
        return materials.get_handle(generate_material_handle_id(material));
    }

    let material_name = generate_name_for_voxels(voxels);

    if let Some(handle_id) = named_materials.get(&material_name) {
        return materials.get_handle(handle_id.clone());
    }

    let new_texture_width = TEXTURE_SIZE * number_of_images_wide;
    let new_texture_height = TEXTURE_SIZE * number_of_images_in_height;
    let mut pixel_buf = Vec::with_capacity((COLOR_SIZE * new_texture_width * new_texture_height) as usize);

    let mut cached_images = HashMap::new();

    for y in 0..new_texture_height {
        let voxel_y = y / TEXTURE_SIZE;
        let original_y = y - voxel_y * TEXTURE_SIZE;
        let row = &voxels[voxel_y as usize];

        for x in 0..number_of_images_wide {
            let voxel = row[x as usize];
            let voxel_material = if settings.debug_textures { material } else { voxel.material };
            let original_image = cached_images.entry(voxel_material)
                .or_insert_with(|| get_basic_image_pixels(voxel_material, settings));

            let start = (original_y * BYTES_IN_ROW) as usize;
            let end = start + BYTES_IN_ROW as usize;
            pixel_buf.extend(&original_image[start..end]);
        }
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

    let handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        ..original_material
    });

    named_materials.add(material_name, handle.id);

    handle
}

fn get_basic_image_pixels(material: Material, settings: &Res<GameSettings>) -> Vec<u8> {
    get_basic_image_for_material(material, settings)
        .pixels()
        .flat_map(|(.., p)| p.0)
        .collect()
}

fn get_basic_image_for_material(voxel_material: Material, settings: &Res<GameSettings>) -> DynamicImage {
    if settings.debug_textures {
        return generate_image_of_random_color();
    }

    const TEXTURE_PATH: &str = "./assets/texture/block/";
    let material_name = get_material_file_name(voxel_material);
    let material_id = format!("{TEXTURE_PATH}{material_name}");

    image::open(material_id).unwrap()
}

fn generate_image_of_random_color() -> DynamicImage {
    let range = Uniform::new(0_u8, 255_u8);
    let mut rng = rand::thread_rng();
    let red = range.sample(&mut rng);
    let green = range.sample(&mut rng);
    let blue = range.sample(&mut rng);
    let color = Rgba::<u8>::from([red, green, blue, 1]);

    let mut img = DynamicImage::new_rgba8(TEXTURE_SIZE, TEXTURE_SIZE);
    for x in 0..TEXTURE_SIZE {
        for y in 0..TEXTURE_SIZE {
            img.put_pixel(x, y, color.clone());
        }
    }

    img
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

fn generate_asset_path(material: Material) -> String {
    format!("texture/block/{}", get_material_file_name(material))
}

pub fn can_merge_materials(m1: Material, m2: Material) -> bool {
    if m1 == m2 {
        return true;
    }

    const NON_GROUPABLE: [Material; 9] = [
        Material::OrangeLight,
        Material::BlueLight,
        Material::Glass,
        Material::JungleLeaves,
        Material::AcaciaLeaves,
        Material::BirchLeaves,
        Material::DarkOakLeaves,
        Material::SpruceLeaves,
        Material::OakLeaves,
    ];

    !NON_GROUPABLE.contains(&m1) && !NON_GROUPABLE.contains(&m2)
}

#[allow(clippy::needless_pass_by_value)]
pub fn setup(mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    let _id = materials.set(
        generate_material_handle_id(Material::OrangeLight), StandardMaterial {
            base_color_texture: Some(asset_server.load(&generate_asset_path(Material::OrangeLight))),
            reflectance: 1.0,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..Default::default()
        });
    let _id = materials.set(generate_material_handle_id(Material::BlueLight), StandardMaterial {
        base_color_texture: Some(asset_server.load(&generate_asset_path(Material::BlueLight))),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });
    let _id = materials.set(generate_material_handle_id(Material::Glass), StandardMaterial {
        base_color_texture: Some(asset_server.load(&generate_asset_path(Material::Glass))),
        reflectance: 1.0,
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    let leaves_material_names = vec![
        Material::OakLeaves,
        Material::SpruceLeaves,
        Material::AcaciaLeaves,
        Material::BirchLeaves,
        Material::JungleLeaves,
        Material::DarkOakLeaves,
        Material::SpruceLeaves,
    ];
    setup_leaves_materials(&mut materials, &asset_server, leaves_material_names);

    let default_material_names = vec![
        Material::SmoothStone, Material::Water, Material::Farmland,
        Material::WhiteTerracotta,
        Material::Unknown, Material::Pumpkin, Material::Hay, Material::DirtPath, Material::Grass,
        Material::Bedrock, Material::Dirt,
        Material::Podzol, Material::CoarseDirt,
        Material::StoneBricks,
        Material::MossyStoneBricks,
        Material::MossyCobblestone,
        Material::CrackedStoneBricks,
        Material::ChiseledStoneBricks,
        Material::Cobblestone,
        Material::Stone,
        Material::OakLog,
        Material::OakPlanks,
        Material::StrippedOakLog,
        Material::AcaciaLog,
        Material::AcaciaPlanks,
        Material::StrippedAcaciaLog,
        Material::BirchLog,
        Material::BirchPlanks,
        Material::StrippedBirchLog,
        Material::JungleLog,
        Material::JunglePlanks,
        Material::StrippedJungleLog,
        Material::DarkOakLog,
        Material::DarkOakPlanks,
        Material::StrippedDarkOakLog,
        Material::SpruceLog,
        Material::SprucePlanks,
        Material::StrippedSpruceLog,
    ];
    setup_default_materials(&mut materials, &asset_server, default_material_names);
}

pub fn setup_default_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    material_names: Vec<Material>,
) {
    for material in material_names {
        let _id = materials.set(
            generate_material_handle_id(material),
            create_default_material(asset_server.load(&generate_asset_path(material))),
        );
    }
}

pub fn setup_leaves_materials(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    material_names: Vec<Material>,
) {
    for material in material_names {
        let _id = materials.set(
            generate_material_handle_id(material),
            StandardMaterial {
                base_color: Color::DARK_GREEN,
                base_color_texture: Some(asset_server.load(&generate_asset_path(material))),
                reflectance: 0.0,
                ..Default::default()
            },
        );
    }
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