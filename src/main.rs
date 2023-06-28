use rand::Rng;
use std::cmp::Ordering;
use std::env;
use std::io;

mod solver;
use solver::solution::Solution;

mod dooal_io;
use dooal_io::reader::IO;
use dooal_io::config::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config : Config = Config::new(&args);

    println!("{:?}", args);

    let s : Solution = Solution::ConvexHull;
    println!("It compiles and runs!");
    
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
