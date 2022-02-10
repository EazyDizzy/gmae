use std::fs::File;

use fastanvil::{Chunk, JavaChunk, RegionBuffer};
use fastnbt::de::from_bytes;

use crate::entity::point::Point;
use crate::entity::voxel::{Material, Voxel};

const EXPORT_DIAPASON: usize = 8;
const LVL_DIR: &str = "./src/level/lvls/";
const CHUNK_SIZE: usize = 16;
const MAX_NEGATIVE_HEIGHT: f32 = 64.0;

pub fn read_level(lvl_name: &str) -> Vec<Voxel> {
    let mut voxels = vec![];
    let path = [LVL_DIR, lvl_name, "/r.0.0.mca"].concat();
    let file = File::open(path)
        .expect(&format!("Can't open file {}", lvl_name));

    let mut region = RegionBuffer::new(file);

    region.for_each_chunk(|chunk_y, chunk_x, data| {
        if chunk_y > EXPORT_DIAPASON || chunk_x > EXPORT_DIAPASON {
            return;
        }
        let chunk: JavaChunk = from_bytes(data.as_slice()).unwrap();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for height in chunk.y_range() {
                    if let Some(block) = chunk.block(x, height, y) {
                        if block.name() != "minecraft:air" {
                            let voxel_y = (chunk_y * CHUNK_SIZE) + x;
                            let voxel_x = (chunk_x * CHUNK_SIZE) + y;
                            let material = match_name_to_material(block.name());
                            let voxel_z = height as f32 + MAX_NEGATIVE_HEIGHT;

                            voxels.push(Voxel::new(
                                Point::new(voxel_x as f32, voxel_y as f32, voxel_z),
                                material,
                            ));
                        }
                    }
                }
            }
        }
    })
        .expect("Cannot proceed chunks");

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
            eprintln!("Unknown block name: {}", name);
            Material::Unknown
        }
    }
}