mod config;

use config::VmConfig;

fn main() {
    let config = VmConfig::load("vm_config.toml");

    println!("{:#?}", config);

    // initialize RAM using config
    let mut memory = vec![0i16; config.memory_size];

    let sp = config.stack_start;
}