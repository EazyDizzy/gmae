use std::fs;
use std::fs::File;
use std::io::Write;

use fastanvil::{Block, Chunk, JavaChunk, RegionBuffer};
use fastnbt::de::from_bytes;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use lib::entity::level::{DayPart, Level};
use lib::entity::point::Point;
use lib::entity::voxel::{Material, Shape, TrianglePrismProperties, Voxel};

const EXPORT_DIAPASON: usize = 8;
const LVL_DIR: &str = "./assets/lvl/";
const CHUNK_SIZE: usize = 16;
const MAX_NEGATIVE_HEIGHT: f32 = 64.0;

fn main() {
    let lvls = fs::read_dir(LVL_DIR).unwrap();

    for lvl in lvls {
        if let Ok(dir) = lvl {
            let lvl_name = dir.file_name();
            let original_lvl_path = format!("{LVL_DIR}{}/r.0.0.mca", lvl_name.to_str().unwrap());

            if let Ok(original_metadata) = fs::metadata(&original_lvl_path) {
                let serialized_lvl_path = format!("{LVL_DIR}{}/lvl.json.gz", lvl_name.to_str().unwrap());
                let converted_metadata = fs::metadata(&serialized_lvl_path);
                let should_rebuild = if let Ok(converted) = converted_metadata {
                    original_metadata.modified().unwrap() > converted.modified().unwrap()
                        // || lvl_name == "debug"
                } else { true };

                if should_rebuild {
                    println!("converting {original_lvl_path}");
                    let lvl = read_level(lvl_name.to_str().unwrap());
                    let lvl_data = serde_json::to_string(&lvl).unwrap();

                    let file = File::create(serialized_lvl_path).unwrap();
                    let mut e = ZlibEncoder::new(file, Compression::best());
                    e.write_all(lvl_data.as_bytes()).unwrap();
                    e.finish().unwrap();
                }
            }
        }
    }
}

fn read_level(lvl_name: &str) -> Level {
    let mut voxels = vec![];
    let path = [LVL_DIR, lvl_name, "/r.0.0.mca"].concat();
    let file = File::open(path)
        .expect(&format!("Can't open file {}", lvl_name));

    let mut region = RegionBuffer::new(file);

    region.for_each_chunk(|chunk_x, chunk_z, data| {
        if chunk_x > EXPORT_DIAPASON || chunk_z > EXPORT_DIAPASON {
            return;
        }
        let chunk: JavaChunk = from_bytes(data.as_slice()).unwrap();

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in chunk.y_range() {
                    if let Some(block) = chunk.block(x, y, z) {
                        if block.name() != "minecraft:air" {
                            let voxel_x = (chunk_x * CHUNK_SIZE) + x;
                            let voxel_z = (chunk_z * CHUNK_SIZE) + z;
                            let material = match_name_to_material(block.name());
                            let shape = detect_shape(block);
                            let voxel_y = y as f32 + MAX_NEGATIVE_HEIGHT;

                            voxels.push(Voxel::new(
                                Point::new(voxel_x as f32, voxel_y, voxel_z as f32),
                                material,
                                shape,
                            ));
                        }
                    }
                }
            }
        }
    })
        .expect("Cannot proceed chunks");

    let day_part = match lvl_name {
        "village" => DayPart::Night,
        &_ => DayPart::Day
    };

    // TODO sort voxels here to remove sorting later
    Level::new(voxels, day_part)
}

fn match_name_to_material(name: &str) -> Material {
    match name {
        "minecraft:glass" | "minecraft:glass_pane" => Material::Glass,
        "minecraft:hay_block" => Material::Hay,
        "minecraft:pumpkin" => Material::Pumpkin,
        "minecraft:white_terracotta" => Material::WhiteTerracotta,
        "minecraft:water" => Material::Water,
        // Light
        "minecraft:glowstone" => Material::OrangeLight,
        "minecraft:sea_lantern" => Material::BlueLight,
        // Ground
        "minecraft:dirt_path" => Material::DirtPath,
        "minecraft:coarse_dirt" | "minecraft:rooted_dirt" => Material::CoarseDirt,
        "minecraft:farmland" => Material::Farmland,
        "minecraft:podzol" => Material::Podzol,
        "minecraft:grass_block" => Material::Grass,
        "minecraft:dirt" => Material::Dirt,
        // Stone
        "minecraft:bedrock" => Material::Bedrock,
        "minecraft:stone" => Material::Stone,
        "minecraft:stone_bricks" | "minecraft:stone_brick_stairs" => Material::StoneBricks,
        "minecraft:smooth_stone" => Material::SmoothStone,
        "minecraft:mossy_cobblestone" => Material::MossyCobblestone,
        "minecraft:mossy_stone_bricks" => Material::MossyStoneBricks,
        "minecraft:cracked_stone_bricks" => Material::CrackedStoneBricks,
        "minecraft:chiseled_stone_bricks" => Material::ChiseledStoneBricks,
        "minecraft:cobblestone" | "minecraft:cobblestone_stairs" => Material::Cobblestone,
        // Wood + Leaves
        "minecraft:stripped_oak_wood" | "minecraft:stripped_oak_log" => Material::StrippedOakLog,
        "minecraft:oak_planks" | "minecraft:oak_stairs" => Material::OakPlanks,
        "minecraft:oak_leaves" => Material::OakLeaves,
        "minecraft:oak_log" => Material::OakLog,
        "minecraft:stripped_spruce_wood" | "minecraft:stripped_spruce_log" => Material::StrippedSpruceLog,
        "minecraft:spruce_leaves" => Material::SpruceLeaves,
        "minecraft:spruce_log" | "minecraft:spruce_wood" => Material::SpruceLog,
        "minecraft:spruce_planks" => Material::SprucePlanks,
        "minecraft:stripped_dark_oak_wood" | "minecraft:stripped_dark_oak_log" => Material::StrippedDarkOakLog,
        "minecraft:dark_oak_leaves" => Material::DarkOakLeaves,
        "minecraft:dark_oak_log" => Material::DarkOakLog,
        "minecraft:dark_oak_planks" => Material::DarkOakPlanks,
        "minecraft:stripped_birch_log" => Material::StrippedBirchLog,
        "minecraft:birch_leaves" => Material::BirchLeaves,
        "minecraft:birch_log" => Material::BirchLog,
        "minecraft:birch_planks" => Material::BirchPlanks,
        "minecraft:stripped_acacia_log" => Material::StrippedAcaciaLog,
        "minecraft:acacia_leaves" => Material::AcaciaLeaves,
        "minecraft:acacia_log" => Material::AcaciaLog,
        "minecraft:acacia_planks" => Material::AcaciaPlanks,
        "minecraft:stripped_jungle_log" => Material::StrippedJungleLog,
        "minecraft:jungle_leaves" => Material::JungleLeaves,
        "minecraft:jungle_log" => Material::JungleLog,
        "minecraft:jungle_planks" => Material::JunglePlanks,
        _ => {
            eprintln!("Unknown block name: {name}");
            Material::Unknown
        }
    }
}

fn detect_shape(block: &Block) -> Shape {
    // render everything as a cube until support for custom polygons added
    if block.name().ends_with("_stairs") {
        let properties = TrianglePrismProperties::from_properties(block.properties());
        Shape::TrianglePrism(properties)
    } else {
        Shape::Cube
    }
}