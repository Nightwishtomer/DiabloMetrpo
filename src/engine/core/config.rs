// src/engine/core/config.rs

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub target_fps: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub debug: bool,
}

impl EngineConfig {
    pub fn load(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .expect("Faild to read config file");
        
        ron::from_str(&data)
            .expect("Faild to parse config")
    }
}