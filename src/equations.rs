use std::fmt;
use std::fmt::Formatter;
use crate::Rational;

pub mod constraint;
pub mod objective;
pub mod variable;

pub struct EqualityDetector {}

impl EqualityDetector {
    pub fn got_equal(string : &String) -> bool {
        string.contains("eq") && !string.contains("geq") && !string.contains("leq")
    }

    pub fn vec_contains_eq_keyword(strings : &Vec<String>) -> bool {
        let keyword : String = String::from("eq");
        strings.into_iter().filter(|x| *x == &keyword).count() > 0
    }
}

pub trait Normalisable {
    fn zero_out(index: i32);
    fn one_out(index: i32);
}

pub trait Insertable {
    fn insert(index: i32, val: Rational);
}

pub trait EquationStringProcessable<T> {
    fn process(string: Vec<String>) -> T;
}
