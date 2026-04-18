// take a command then translate it into code of assembly

use crate::command_parser::Cmd;
use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file : File,
}

impl CodeWriter {
    pub fn new(f : File)-> Self{
        Self{
            file: f
        }
    }

    pub fn write_line(&mut self, cmd: Cmd) -> std::io::Result<()>  {
        self.file.write_all(&cmd.comment.into_bytes())?;
        self.file.write_all(b"\n");
        Ok(())
    }
}




