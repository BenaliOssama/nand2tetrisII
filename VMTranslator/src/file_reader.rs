use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub struct FileReader {
    file: Lines<BufReader<File>>,
}

impl FileReader {
    pub fn new(file_name: &str) -> io::Result<Self> {
        let file = File::open(file_name)?;
        let reader = io::BufReader::new(file);

        return Ok(Self {
            file: reader.lines(),
        });
    }

    pub fn next_line(&mut self) -> Option<io::Result<String>> {
        self.file.next()
    }
}
