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
    pub comment: String,
    pub inst: Inst,
    // pub segment:
}

impl Cmd {
    pub fn parse_command(s: &str, line_num: usize) -> Option<Cmd> {
        use Inst::*;
        use Segment::*;
        let components: Vec<&str> = s.trim().split_whitespace().collect();

        // expecting 3 parts in every line
        /*if components.len() != 3 {
            eprintln!("found line with more than 3 parts, not supported yet");
            eprintln!(" line : {}", line_num);
            process::exit(1);
        }*/
        // 
        match components[0] {
            // memory acess commands
            "push" | "pop"  => {
                let (first , second) = Cmd::parse_arg(&components[1..], line_num);

                let inst = if components[0] == "push" {
                    Push(first, second)
                }else{
                    Pop(first, second)
                };

                 
                return Some(Cmd { 
                    comment: String::from("//") + s,
                    inst,
                });
            }

            _ => None,
        }
    }

    pub fn parse_arg( components : &[&str], line_num: usize ) -> (Segment, u16)  {
        use Segment::*;
        match components[0] {
            "constant" => {
                let cons = match components[1].parse::<u16>() {
                    Ok(cons) => cons,
                    Err(e) => {
                        eprintln!("vm error line {}, {e}", line_num);
                        process::exit(1);
                    },
                };

                return (Constant, cons)
            }
            _ => process::exit(1),
        }
    }
}
