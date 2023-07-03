use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;
use std::process;

mod solver;
use solver::solution::Solution;

mod dooal_io;
use dooal_io::reader::IO;
use dooal_io::config::Config;

mod numbers;
use numbers::rationals::rational::Rational;

use crate::dooal_io::parser::Parser;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); // delete in production

    let config: Config = Config::new(&args)
        .unwrap_or_else(|err: &str| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    let contents: Vec<String> = IO::read_file(config.filename);
    println!("{:#?}", contents);

    Parser::process(contents);

    test();

    //let s : Solution = Solution::ConvexHull;
    println!("It compiles and runs!");
    
}

fn test() {
    let r: Rational = Rational::new(3, 2).unwrap_or_else(|_err| {
        print!("Rational got problem");
        process::exit(1);
    });

    let s = Rational {numerator: 2, denominator: 8};
    println!("{}",-s);
}

