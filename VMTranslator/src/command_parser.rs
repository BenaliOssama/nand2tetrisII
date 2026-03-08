enum Inst {
    PUSH,
    POP,
}

pub struct Cmd {
    inst: Inst,
}

impl Cmd {
    pub fn new(s: &str) -> Result<Cmd, String> {
        todo!()
    }

    pub fn parse_command(s: &str) {
        todo!()
    }
}
