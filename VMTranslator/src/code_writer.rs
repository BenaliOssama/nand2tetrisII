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
        use crate::command_parser::{Inst, Segment};

        if self.emit_source_comments {
            self.file.write_all(cmd.comment.as_bytes())?;
            self.file.write_all(b"\n")?;
        }

        match cmd.inst {
            Inst::Push(Segment::Constant, val) => {
                let asm = format!("@{val}\nD=A\n@SP\nA=M\nM=D\n@SP\nM=M+1\n");
                self.file.write_all(asm.as_bytes())?;
            }
            Inst::Push(Segment::Local, val) => {
                let asm = format!("@LCL\nD=M\n@{val}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nD=M+1\nM=D\n");
                self.file.write_all(asm.as_bytes())?;
            }
            Inst::Push(Segment::Argument, val) => {
                let asm = format!("@ARG\nD=M\n@{val}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nD=M+1\nM=D\n");
                self.file.write_all(asm.as_bytes())?;
            }
            Inst::Push(Segment::This, val) => {
                let asm = format!("@THIS\nD=M\n@{val}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nD=M+1\nM=D\n");
                self.file.write_all(asm.as_bytes())?;
            }
            Inst::Push(Segment::That, val) => {
                let asm = format!("@THAT\nD=M\n@{val}\nA=A+D\nD=M\n@SP\nA=M\nM=D\n@SP\nD=M+1\nM=D\n");
                self.file.write_all(asm.as_bytes())?;
            }
            _ => todo!("other instructions not yet implemented"),
        }

        Ok(())
    }
}




