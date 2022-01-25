use std::fs::File;
use std::io;

use fastanvil::{Chunk, HeightMode, JavaChunk, RegionBuffer};
use fastnbt::de::from_bytes;
use fastnbt::stream::{Parser, Value};

use crate::entity::point::Point;
use crate::entity::voxel::{Voxel, VoxelMaterial};

pub fn read_level(lvl_name: &str) -> Vec<Voxel> {
    let mut voxels = vec![];
    let path = ["./src/level/lvls/", lvl_name, "/r.0.0.mca"].concat();
    let file = File::open(path)
        .expect(&format!("Can't open file {}", lvl_name));

    let mut region = RegionBuffer::new(file);

    region.for_each_chunk(|chunk_y, chunk_x, data| {
        if chunk_y > 4 || chunk_x > 4 {
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
                            let material = match block.name() {
                                "minecraft:bedrock" => { VoxelMaterial::Bedrock }
                                "minecraft:grass_block" => { VoxelMaterial::Grass }
                                "minecraft:dirt" => { VoxelMaterial::Dirt }
                                "minecraft:stone" => { VoxelMaterial::Stone }
                                &_ => {
                                    dbg!(block.name());
                                    VoxelMaterial::Unknown
                                }
                            };

                            voxels.push(Voxel {
                                material,
                                position: Point::new(
                                    voxel_x as isize,
                                    voxel_y as isize,
                                    height + 64,
                                ),
                            });
                        }
                    }
                }
            }
        }
    });

    voxels
}