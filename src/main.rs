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

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); // delete in production

    let config: Config = Config::new(&args)
        .unwrap_or_else(|err: &str| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    let contents: String = IO::read_file(config.filename);
    println!("{}", contents);

    test();

    let s : Solution = Solution::ConvexHull;
    println!("It compiles and runs!");
    
}

fn test() {
    let r: Rational = Rational::new(-16, 32).unwrap_or_else(|_err| {
        print!("Rational got problem");
        process::exit(1);
    });
    println!("{}",r);
}

// Dead code here for experiments
fn number_guesser() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
