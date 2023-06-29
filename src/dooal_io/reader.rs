use std::{fs::{self, File}, io::{self, BufRead}, path::Path};
pub struct IO {
    name:String
}

impl IO {
    pub fn read_file(filepath: String) -> Vec<String> {
        let mut lines_in_file : Vec<String> = Vec::new();
        if let Ok(lines) = Self::read_lines(filepath) {
            for line in lines {
                if let Ok(ip) = line {
                    lines_in_file.push(ip);
                }
            }
        }
        // fs::read_to_string(filepath)
        // .expect("Failed to read file")
        lines_in_file
    }

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
        where P: AsRef<Path> {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }
}

