use std::fs::{read_to_string, File};
use std::io::Write;

struct CodeWriter {
    output: File,
    label_counter: usize,
}

impl CodeWriter {
    fn new(path: &str) -> Self {
        Self {
            output: File::create(path).unwrap(),
            label_counter: 0,
        }
    }

    fn write_line(&mut self, s: &str) {
        writeln!(self.output, "{}", s).unwrap();
    }

    fn write_arithmetic(&mut self, cmd: &str) {
        match cmd {
            "add" => {
                self.write_line("@SP");
                self.write_line("AM=M-1");
                self.write_line("D=M");
                self.write_line("A=A-1");
                self.write_line("M=M+D");
            }
            "sub" => {
                self.write_line("@SP");
                self.write_line("AM=M-1");
                self.write_line("D=M");
                self.write_line("A=A-1");
                self.write_line("M=M-D");
            }
            _ => {}
        }
    }

    fn write_push_constant(&mut self, value: &str) {
        self.write_line(&format!("@{}", value));
        self.write_line("D=A");
        self.write_line("@SP");
        self.write_line("A=M");
        self.write_line("M=D");
        self.write_line("@SP");
        self.write_line("M=M+1");
    }
}

fn main() {
    let input = read_to_string("SimpleAdd.vm").unwrap();
    let mut writer = CodeWriter::new("SimpleAdd.asm");

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "push" if parts[1] == "constant" => {
                writer.write_push_constant(parts[2]);
            }
            "add" | "sub" => {
                writer.write_arithmetic(parts[0]);
            }
            _ => {}
        }
    }
}