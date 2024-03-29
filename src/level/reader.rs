use std::fs::File;
use std::io::Read;

use flate2::read::ZlibDecoder;
use lib::entity::level::Level;
use mca_cuboids::BlockSequence;

const LVL_DIR: &str = "./assets/lvl/";

pub fn read_level(lvl_name: &str) -> Level {
    let path = [LVL_DIR, lvl_name, "/lvl.json.gz"].concat();
    // TODO normal error handling with modal showing to user
    let lvl_file = File::open(path).expect("Can't open file");

    let mut decoder = ZlibDecoder::new(lvl_file);
    let mut json = String::new();
    decoder
        .read_to_string(&mut json)
        .expect("Failed to decode lvl");

    serde_json::from_str(&json).expect("Failed to parse lvl")
}

pub fn read_level_collisions(lvl_name: &str) -> Vec<BlockSequence> {
    let path = [LVL_DIR, lvl_name, "/collisions.json"].concat();
    let mut json = String::new();
    File::open(path)
        .expect("Can't open collisions file")
        .read_to_string(&mut json)
        .expect("Cannot read collisions from file");

    serde_json::from_str(&json).expect("Failed to parse lvl")
}

#[cfg(test)]
#[bench]
fn bench_lvl_deserialize(b: &mut test::Bencher) {
    b.iter(|| {
        read_level("debug");
    });
}
