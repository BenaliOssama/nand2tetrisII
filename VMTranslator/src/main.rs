mod code_writer;
mod command_parser;
mod file_reader;

use command_parser::Cmd;
use file_reader::FileReader;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut file_reader = FileReader::new(&args[1])?;

    while let Some(result) = file_reader.next_line() {
        let line = result?;
        let cmd = Cmd::parse_command(&line); //.unwrap();
        match cmd {
            Some(cmd) => {
                println!("{:?}", cmd);
            }
            None => (),
        }
    }
    Ok(())
}
