use super::flags::{Flag, Help};

pub struct Config {
    query: Flag,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let flag : String = args[1].clone();

        let query = match flag.as_str() {
            "-h" => Flag::Help,
            "-s" => Flag::Solve,
            "-c" => Flag::Convert,
            _ => panic!("Invalid Flag Parsed!")
        };

        let filename : String = args[2].clone();

        Ok(Config {query, filename})
    }

    fn check_query(args: &[String]) -> bool {
        if args.len() < 3 {
            return false;
        }
        return true;
    }
}