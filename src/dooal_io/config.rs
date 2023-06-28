pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query : String = args[1].clone();
        let filename : String = args[2].clone();

        Ok(Config {query, filename})
    }
}