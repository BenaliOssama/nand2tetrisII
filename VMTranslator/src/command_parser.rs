use std::fmt::Debug;
use std::process;

#[derive(Debug)]
pub enum Inst {
    Push(Segment, u16),
    Pop(Segment, u16),

    Add,
    Sub,
    Neg,

    Eq,
    Gt,
    Lt,

    And,
    Or,
    Not,
}


#[derive(Debug)]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}


#[derive(Debug)]
pub struct Cmd {
    pub inst: Inst,
    // pub segment:
}

impl Cmd {
    pub fn parse_command(s: &str) -> Option<Cmd> {
        use Inst::*;
        use Segment::*;
        let components: Vec<&str> = s.trim().split_whitespace().collect();
        match components[0] {
            "push" => {
                match components[1] {
                    "constant" => {
                        let Ok(cons) = components[2].parse::<u16>()  else {
                            process::exit(1);
                        };
                        return Some(Cmd {
                            inst: Push(Constant, cons),
                        });
                    }
                    _ => None,
                }
            }
            "pop" => {
                return Some(Cmd {
                    inst: Pop(Argument, 0),
                });
            }
            _ => None,
        }
    }
}
