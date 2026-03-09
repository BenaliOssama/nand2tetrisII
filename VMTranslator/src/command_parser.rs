use std::fmt::Debug;

#[derive(Debug)]
pub enum Inst {
    PUSH,
    POP,
}
#[derive(Debug)]
pub struct Cmd {
    pub inst: Inst,
}

impl Cmd {
    pub fn parse_command(s: &str) -> Option<Cmd> {
        use Inst::*;
        let components: Vec<&str> = s.trim().split_whitespace().collect();
        match components[0] {
            "push" => {
                return Some(Cmd { inst: PUSH });
            }
            "pop" => {
                return Some(Cmd { inst: POP });
            }
            _ => None,
        }
    }
}
