use std::fs::File;
use std::io::Read;

use flate2::read::ZlibDecoder;
use lib::entity::level::Level;

const LVL_DIR: &str = "./assets/lvl/";

pub fn read_level(lvl_name: &str) -> Level {
    let path = [LVL_DIR, lvl_name, "/lvl.json.gz"].concat();
    let lvl_file = File::open(path).expect("Can't open file");

    let mut decoder = ZlibDecoder::new(lvl_file);
    let mut json = String::new();
    decoder.read_to_string(&mut json)
        .expect("Failed to decode lvl");

    serde_json::from_str(&json)
        .expect("Failed to parse lvl")
}