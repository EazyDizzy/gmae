use std::fs::File;

use fastanvil::{Chunk, HeightMode, JavaChunk, RegionBuffer};
use fastnbt::de::from_bytes;
use rand::Rng;

use crate::ENABLE_EXTREME_GRAPHIC;
use crate::entity::point::Point;
use crate::entity::voxel::{Voxel, Material};
use crate::level::util::get_rng;

const EXPORT_DIAPASON: usize = 1;

pub fn read_level(lvl_name: &str) -> Vec<Voxel> {
    let mut voxels = vec![];
    let path = ["./src/level/lvls/", lvl_name, "/r.0.0.mca"].concat();
    let file = File::open(path)
        .expect(&format!("Can't open file {}", lvl_name));

    let mut region = RegionBuffer::new(file);
    let mut rng = get_rng();
    let mut voxel_id = 0;

    region.for_each_chunk(|chunk_y, chunk_x, data| {
        if chunk_y > EXPORT_DIAPASON || chunk_x > EXPORT_DIAPASON {
            return;
        }
        let chunk: JavaChunk = from_bytes(data.as_slice()).unwrap();

        for x in 0..16 {
            for y in 0..16 {
                let max_height = chunk.surface_height(x, y, HeightMode::Trust);
                let min_height = chunk.y_range().start;

                for height in min_height..max_height {
                    if let Some(block) = chunk.block(x, height, y) {
                        if block.name() != "minecraft:air" {
                            let voxel_y = (chunk_y * 16) + x;
                            let voxel_x = (chunk_x * 16) + y;
                            let material = match_name_to_material(block.name());
                            let voxel_z = if should_randomize_voxel_z(block.name()) {
                                let h = height as f32 + 64.0;
                                *rng.choose(&[h, h - 0.05, h + 0.05, h - 0.1, h + 0.1]).unwrap()
                            } else {
                                height as f32 + 64.0
                            };

                            voxels.push(Voxel::new(
                                voxel_id,
                                Point::new(
                                    voxel_x as isize,
                                    voxel_y as isize,
                                    voxel_z,
                                ),
                                material,
                            ));
                            voxel_id += 1;
                        }
                    }
                }
            }
        }
    }).unwrap();

    voxels
}

fn match_name_to_material(name: &str) -> Material {
    match name {
        "minecraft:bedrock" => { Material::Bedrock }
        "minecraft:grass_block" => { Material::Grass }
        "minecraft:dirt" => { Material::Dirt }
        "minecraft:stone" => { Material::Stone }
        "minecraft:oak_planks" => { Material::WoodenPlanks }
        "minecraft:glowstone" => { Material::OrangeLight }
        "minecraft:sea_lantern" => { Material::BlueLight }
        "minecraft:dirt_path" => { Material::DirtPath }
        "minecraft:glass" => { Material::Glass }
        "minecraft:hay_block" => { Material::Hay }
        "minecraft:pumpkin" => { Material::Pumpkin }
        &_ => {
            println!("Unknown block name: {}", name);
            Material::Unknown
        }
    }
}

fn should_randomize_voxel_z(name: &str) -> bool {
    ENABLE_EXTREME_GRAPHIC && ["minecraft:dirt_path", "minecraft:grass_block"].contains(&name)
}