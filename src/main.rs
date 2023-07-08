use std::env;
use std::io;
use std::process;

mod solver;
use solver::solution::Solution;

mod dooal_io;
use dooal_io::reader::IO;
use dooal_io::config::Config;

mod numbers;
mod equations;

use numbers::rationals::rational::Rational;

use crate::dooal_io::parser::Parser;
use crate::equations::constraint::Constraint;

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

    let t = Rational::from_string(String::from("-1.503"));
    println!("{}", t);

    let u = Rational::from_string(String::from("3.141592"));
    println!("{}", u);

    let v = Rational::from_string(String::from("0.61729"));
    println!("{}", v);

    let w = Rational::from_string(String::from("-0.61729"));
    println!("{}", w);

    let x = Rational::from_string(String::from("-3.141592"));
    println!("{}", x);

    let y = Rational::from_string(String::from("-3"));
    println!("{}", y);

    let z = Rational::from_string(String::from("5"));
    println!("{}", z);

    let vector = vec![t, u, v, w, x, y, z];
    println!("{:?}", Constraint::negate_all(vector));
}

