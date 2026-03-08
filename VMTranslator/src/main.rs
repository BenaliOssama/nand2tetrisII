mod code_writer;
mod file_reader;

use file_reader::FileReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_reader = FileReader::new(&args[1]);
}
