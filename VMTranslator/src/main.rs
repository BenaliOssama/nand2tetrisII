mod code_writer;
mod command_parser;
mod config;
mod file_reader;

use command_parser::Cmd;
use file_reader::FileReader;
use code_writer::CodeWriter;
use std::fs::File;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("run with file.vm".into())
    }

    let cfg = config::Config::load("config.toml")?;

    let mut file_reader = FileReader::new(&args[1])?;
    let output_path = std::path::Path::new(&args[1])
        .with_extension(&cfg.output.extension);
    let file = File::create(&output_path)?;
    let mut code_writer = CodeWriter::new(file, cfg.output.emit_source_comments);

    let mut line_number =  0;
    while let Some(result) = file_reader.next_line() {
        line_number += 1;
        let line = result?;
        let cmd = Cmd::parse_command(&line, line_number); //.unwrap();
        match cmd {
            Some(cmd) => {
                println!("{:?}", cmd);
                code_writer.write_line(cmd);
            }
            None => (),
        }
    }
    Ok(())
}
