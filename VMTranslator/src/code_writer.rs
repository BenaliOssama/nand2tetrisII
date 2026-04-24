// take a command then translate it into code of assembly

use crate::command_parser::Cmd;
use std::fs::File;
use std::io::Write;

pub struct CodeWriter {
    file: File,
    emit_source_comments: bool,
}

impl CodeWriter {
    pub fn new(f: File, emit_source_comments: bool) -> Self {
        Self {
            file: f,
            emit_source_comments,
        }
    }

    pub fn write_line(&mut self, cmd: Cmd) -> std::io::Result<()> {
        if self.emit_source_comments {
            self.file.write_all(cmd.comment.as_bytes())?;
            self.file.write_all(b"\n")?;
        }

        let str = format!("
            @{cmd.}
            D=A
            @SP
            A=M
            M=D
                          ")
        Ok(())
    }
}




