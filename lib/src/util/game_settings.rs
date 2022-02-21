use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    pub debug_textures: bool,
}

impl GameSettings {
    pub fn from_file(path: &str) -> GameSettings {
        let mut json = String::new();
        File::open(path).expect("Can't open config file")
            .read_to_string(&mut json).expect("Can't read config file");

        serde_json::from_str(&json)
            .expect("Failed to parse lvl")
    }
}