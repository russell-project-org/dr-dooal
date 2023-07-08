use std::fmt;
use std::fmt::Formatter;
use crate::Rational;

pub mod constraint;
pub mod objective;
pub mod variable;

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
