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
                            let shape = detect_shape(block);
                            let voxel_z = height as f32 + MAX_NEGATIVE_HEIGHT;

                            voxels.push(Voxel::new(
                                Point::new(voxel_x as f32, voxel_y as f32, voxel_z),
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
        "debug" | "village" => DayPart::Night,
        &_ => DayPart::Day
    };

    // TODO sort voxels here to remove sorting later
    Level::new(voxels, day_part)
}

fn match_name_to_material(name: &str) -> Material {
    match name {
        "minecraft:bedrock" => { Material::Bedrock }
        "minecraft:grass_block" => { Material::Grass }
        "minecraft:dirt" => { Material::Dirt }
        "minecraft:stone" => { Material::Stone }
        "minecraft:oak_planks" | "minecraft:oak_stairs" => { Material::WoodenPlanks }
        "minecraft:glowstone" => { Material::OrangeLight }
        "minecraft:sea_lantern" => { Material::BlueLight }
        "minecraft:dirt_path" => { Material::DirtPath }
        // TODO change?
        "minecraft:glass_pane" => { Material::Glass }
        "minecraft:glass" => { Material::Glass }
        "minecraft:hay_block" => { Material::Hay }
        "minecraft:pumpkin" => { Material::Pumpkin }
        "minecraft:cobblestone" | "minecraft:cobblestone_stairs" => { Material::Cobblestone }
        "minecraft:mossy_cobblestone" => { Material::MossyCobblestone }
        "minecraft:oak_leaves" => { Material::OakLeaves }
        "minecraft:spruce_log" | "minecraft:spruce_wood" => { Material::SpruceLog }
        "minecraft:stripped_spruce_wood" => { Material::StrippedSpruceLog }
        "minecraft:oak_log" => { Material::OakLog }
        "minecraft:white_terracotta" => { Material::WhiteTerracotta }
        "minecraft:farmland" => { Material::Farmland }
        "minecraft:stripped_oak_wood" | "minecraft:stripped_oak_log" => { Material::StrippedOakLog }
        "minecraft:water" => { Material::Water }
        "minecraft:smooth_stone" => { Material::SmoothStone }
        "minecraft:spruce_leaves" => { Material::SpruceLeaves }
        "minecraft:stripped_dark_oak_wood" => { Material::StrippedDarkOakLog }
        "minecraft:podzol" => { Material::Podzol }
        "minecraft:coarse_dirt" | "minecraft:rooted_dirt" => { Material::CoarseDirt }
        "minecraft:stone_bricks" => { Material::StoneBricks }
        &_ => {
            eprintln!("Unknown block name: {name}");
            Material::Unknown
        }
    }
}

fn detect_shape(block: &Block) -> Shape {
    // render everything as a cube until support for custom polygons added
    if block.name().ends_with("_stairs") && false {
        let properties = TrianglePrismProperties::from_properties(block.properties());
        Shape::TrianglePrism(properties)
    } else {
        Shape::Cube
    }
}