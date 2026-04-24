use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub vm: Vm,
    pub output: Output,
}

#[derive(Deserialize)]
pub struct Vm {
    pub stack_base: u16,
    pub segments: Segments,
    pub registers: Registers,
}

#[derive(Deserialize)]
pub struct Segments {
    pub local: u16,
    pub argument: u16,
    pub this: u16,
    pub that: u16,
    pub temp_base: u16,
    pub pointer_base: u16,
    pub static_base: u16,
    pub static_end: u16,
}

#[derive(Deserialize)]
pub struct Registers {
    pub sp: u16,
    pub lcl: u16,
    pub arg: u16,
    pub this: u16,
    pub that: u16,
}

#[derive(Deserialize)]
pub struct Output {
    pub extension: String,
    pub emit_source_comments: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&text)?;
        Ok(config)
    }
}
