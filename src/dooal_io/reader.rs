use std::fs;
pub struct IO {
    name:String
}

impl IO {
    pub fn read_file(filepath: String) -> String {
        fs::read_to_string(filepath)
        .expect("Failed to read file")
    }
}

