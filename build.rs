use std::fs;
use std::fs::File;
use std::io::Write;

use bevy_internal::math::vec3;
use fastanvil::{Block, Chunk, CurrentJavaChunk, Region};
use fastnbt::from_bytes;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use lib::entity::level::creature::{Creature, CreatureName};
use lib::entity::level::{DayPart, Level};
use lib::entity::voxel::{Material, Shape, TrianglePrismProperties, Voxel};
use mca_cuboids::{BlockCoordinates, ExportParams};

const EXPORT_DIAPASON: usize = 8;
const LVL_DIR: &str = "./assets/lvl/";
const CHUNK_SIZE: usize = 16;
const MAX_NEGATIVE_HEIGHT: f32 = 64.0;

fn main() {
    let lvls = fs::read_dir(LVL_DIR).expect("Cannot read files from lvls dir.");

    for dir in lvls.flatten() {
        let file_name = dir.file_name();
        let lvl_name = file_name.to_str().unwrap();
        let original_lvl_path = format!("{LVL_DIR}{}/r.0.0.mca", lvl_name);

        if let Ok(original_metadata) = fs::metadata(&original_lvl_path) {
            let converted_lvl_path = format!("{LVL_DIR}{}/lvl.json.gz", lvl_name);
            let converted_metadata = fs::metadata(&converted_lvl_path);
            let should_rebuild = if let Ok(converted) = converted_metadata {
                original_metadata.modified().unwrap() > converted.modified().unwrap()
                    // || lvl_name == "debug"
            } else {
                true
            };

            if should_rebuild {
                println!("Converting {original_lvl_path}");

                {
                    let mut collisions = mca_cuboids::export_cuboids(
                        &dir.path().to_str().unwrap(),
                        ExportParams {
                            start: BlockCoordinates::new(0, -64, 0),
                            end: BlockCoordinates::new(64, 0, 64),
                            skip_blocks: vec![
                                "minecraft:flowering_azalea".to_owned(),
                                "minecraft:grass".to_owned(),
                                "minecraft:oxeye_daisy".to_owned(),
                            ],
                        },
                    )
                    .expect("Failed to build collisions");
                    collisions.iter_mut().for_each(|seq| {
                        seq.start.y += 64;
                        seq.end.y += 64;
                    });
                    println!("collisions: {}", collisions.len());

                    let serialized_collisions =
                        serde_json::to_string(&collisions).expect("Cannot serialize collisions.");
                    let mut file = File::create(format!("{LVL_DIR}{}/collisions.json", lvl_name))
                        .expect("Cannot create file for lvl saving.");
                    file.write_all(serialized_collisions.as_bytes())
                        .expect("Cannot write collisions to file.");
                }
                {
                    let lvl = read_level(lvl_name);
                    let serialized_lvl =
                        serde_json::to_string(&lvl).expect("Cannot serialize lvl.");
                    let file = File::create(converted_lvl_path)
                        .expect("Cannot create file for lvl saving.");
                    let mut e = ZlibEncoder::new(file, Compression::best());
                    e.write_all(serialized_lvl.as_bytes())
                        .expect("Cannot write lvl to file.");
                    e.finish().expect("Cannot finish writing lvl to file.");
                }
            }
        } else {
            eprintln!("Cannot read metadata of {}", &original_lvl_path);
        }
    }
}

fn read_level(lvl_name: &str) -> Level {
    let mut voxels = vec![];
    let mut creatures = vec![];
    let path = [LVL_DIR, lvl_name, "/r.0.0.mca"].concat();
    let file = File::open(path).unwrap_or_else(|_| panic!("Can't open file {}", lvl_name));

    let mut region = Region::from_stream(file).expect("Cannot create region from file.");

    region.iter().flatten().for_each(|chunk| {
        let chunk_x = chunk.x;
        let chunk_z = chunk.z;
        let data = chunk.data;
        if chunk_x > EXPORT_DIAPASON || chunk_z > EXPORT_DIAPASON {
            return;
        }
        let chunk: CurrentJavaChunk =
            from_bytes(data.as_slice()).expect("Cannot parse chunk data.");

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for y in chunk.y_range() {
                    if let Some(block) = chunk.block(x, y, z) {
                        if block.name() != "minecraft:air" {
                            let voxel_x = (chunk_x * CHUNK_SIZE) + x;
                            let voxel_z = (chunk_z * CHUNK_SIZE) + z;
                            let voxel_y = y as f32 + MAX_NEGATIVE_HEIGHT;
                            let point = vec3(voxel_x as f32, voxel_y, voxel_z as f32);
                            match block.name() {
                                "minecraft:oak_sign" => {
                                    creatures.push(Creature::neytral(CreatureName::Dummy, point));
                                    continue;
                                }
                                "minecraft:spruce_sign" => {
                                    creatures.push(Creature::enemy(CreatureName::Pizza, point));
                                    continue;
                                }
                                _ => {}
                            }

                            let material = match_name_to_material(block.name());
                            if material != Material::Unknown {
                                let shape = detect_shape(block);
                                voxels.push(Voxel::new(point, material, shape));
                            }
                        }
                    }
                }
            }
        }
    });

    let day_part = DayPart::Night;

    // TODO sort voxels here to remove sorting later
    println!("creatures: {}", creatures.len());
    println!("voxels: {}", voxels.len());
    Level::new(lvl_name.to_string(), voxels, day_part, creatures)
}

fn match_name_to_material(name: &str) -> Material {
    match name {
        "minecraft:glass" | "minecraft:glass_pane" |
        "minecraft:hay_block" |
        "minecraft:pumpkin" |
        "minecraft:spruce_fence" |
        "minecraft:iron_bars" |
        "minecraft:white_terracotta" |
        // Ground
        "minecraft:dirt_path" |
        "minecraft:coarse_dirt" | "minecraft:rooted_dirt" |
        "minecraft:farmland" |
        "minecraft:podzol" |
        "minecraft:grass_block" |
        "minecraft:dirt" |
        // Stone
        "minecraft:bedrock" |
        "minecraft:stone" |
        "minecraft:stone_bricks" | "minecraft:stone_brick_stairs" |
        "minecraft:smooth_stone" |
        "minecraft:mossy_cobblestone" |
        "minecraft:mossy_stone_bricks" |
        "minecraft:cracked_stone_bricks" |
        "minecraft:chiseled_stone_bricks" |
        "minecraft:cobblestone" | "minecraft:cobblestone_stairs" |
        // Wood + Leaves
        "minecraft:stripped_oak_wood" | "minecraft:stripped_oak_log" |
        "minecraft:oak_planks" | "minecraft:oak_stairs" |
        "minecraft:oak_log" |
        "minecraft:stripped_spruce_wood" | "minecraft:stripped_spruce_log" |
        "minecraft:spruce_log" | "minecraft:spruce_wood" |
        "minecraft:spruce_planks" |
        "minecraft:stripped_dark_oak_wood" | "minecraft:stripped_dark_oak_log" |
        "minecraft:dark_oak_log" |
        "minecraft:dark_oak_planks" |
        "minecraft:stripped_birch_log" |
        "minecraft:birch_log" |
        "minecraft:birch_planks" |
        "minecraft:stripped_acacia_log" |
        "minecraft:acacia_log" |
        "minecraft:acacia_planks" |
        "minecraft:stripped_jungle_log" |
        "minecraft:jungle_log" |
        "minecraft:jungle_planks"  => Material::Solid,
        "minecraft:oak_leaves" |
        "minecraft:spruce_leaves" |
        "minecraft:dark_oak_leaves" |
        "minecraft:birch_leaves" |
        "minecraft:acacia_leaves" |
        "minecraft:jungle_leaves"  => Material::Passable,
        // Light
        "minecraft:glowstone" => Material::OrangeLight,
        "minecraft:sea_lantern" => Material::BlueLight,
        "minecraft:water" => Material::Water,
        _ => {
            eprintln!("Unknown block name: {name}");
            Material::Unknown
        }
    }
}

fn detect_shape(block: &Block) -> Shape {
    // TODO implement stairs collisions
    if block.name().ends_with("_stairs") {
        let properties = TrianglePrismProperties::from_properties(block.properties());
        Shape::TrianglePrism(properties)
    } else {
        Shape::Cube
    }
}
