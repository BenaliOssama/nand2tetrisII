struct RAM {
    memory: Vec<i16>,      // full RAM
    sp: usize,             // stack pointer stored in ram 0, starts at 258
    local: usize,          // base of LCL
    argument: usize,       // base of ARG
    this: usize,           // base of THIS
    that: usize,           // base of THAT
    temp_base: usize,      // base of TEMP (fixed)
}


impl RAM {
    fn push(&mut self, value: i16) {
        self.memory[self.sp] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> i16 {
        self.sp -= 1;
        self.memory[self.sp]
    }
}

impl RAM {
    fn set_segment(&mut self, segment: &str, index: usize, value: i16) {
        let addr = match segment {
            "local" => self.local + index,
            "argument" => self.argument + index,
            "this" => self.this + index,
            "that" => self.that + index,
            "temp" => self.temp_base + index,
            _ => panic!("Unknown segment"),
        };
        self.memory[addr] = value;
    }

    fn get_segment(&self, segment: &str, index: usize) -> i16 {
        let addr = match segment {
            "local" => self.local + index,
            "argument" => self.argument + index,
            "this" => self.this + index,
            "that" => self.that + index,
            "temp" => self.temp_base + index,
            _ => panic!("Unknown segment"),
        };
        self.memory[addr]
    }
}