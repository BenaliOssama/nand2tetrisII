use std::fs::File;
use std::io::{self, BufRead};



pub struct VMCommand {
    command: Command,
    Arg1: Option<Arg>,
    Arg2: OPtion<Arg>,
}

struct Command {
    name: String,
}

struct Arg {
    name: String,
}

pub struct FileReader {
    file: io::BufRead,
}
impl VMCommand{
    fn new(next_line: Result<String>){
        // if result is ok ... 
        // parse the line and figure out what it is 
    }
}
impl FileReader{
    fn::new(file_name: &str) -> Result<FileReader> {

        let file = File::open("big_file.txt")?;
        let reader = io::BufReader::new(file);

        return Self {
            file: reader,
        }
    }

    fn next_line(&self) -> Result<String> {
        self.next()
    }

    fn next_command(&self) -> Result<VMCommand>{
        let next_line = self.next_line();
        let command = VMCommand::New(next_line);
        return command;
    }
}

