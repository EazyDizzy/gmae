use std::fs::File;
use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    background_music_volume: Option<f64>,
}

impl GameSettings {
    pub fn from_file(path: &str) -> GameSettings {
        let mut json = String::new();
        File::open(path)
            .expect("Can't open config file")
            .read_to_string(&mut json)
            .expect("Can't read config file");

        serde_json::from_str(&json).expect("Failed to parse lvl")
    }

    pub fn get_background_music_volume(&self) -> f64 {
        self.background_music_volume.unwrap_or(1.0)
    }

    pub fn update_background_music_volume(&mut self, new_value: f64) {
        self.background_music_volume = Some(new_value);
    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(self).unwrap();

        let mut file = File::create("game_settings.json").unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}
