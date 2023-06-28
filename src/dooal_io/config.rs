pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query : String = args[1].clone();
        let filename : String = args[2].clone();

        Config {query, filename}
    }
}