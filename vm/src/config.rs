use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct VmConfig {
    pub memory_size: usize,
    pub stack_start: usize,
    pub segments: Segments,
}

#[derive(Debug, Deserialize)]
pub struct Segments {
    pub local: usize,
    pub argument: usize,
    pub this: usize,
    pub that: usize,
    pub temp: usize,
    pub static_seg: usize,
}

impl VmConfig {
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .expect("Failed to read config file");

        toml::from_str(&content)
            .expect("Failed to parse TOML config")
    }
}