use std::fmt::{Formatter, Result, Display};
use crate::equations::EquationStringProcessable;
use crate::Rational;

pub struct Constraint {
    pub constraint : Vec<Rational>,
    pub leq : bool,
    pub pos_slacks : i32,
    pub neg_slacks : i32
}

impl Constraint {
    pub fn new(vec_num: Vec<String>) -> Constraint {
        let mut row : Vec<Rational> = Vec::new();
        Constraint {constraint : row, leq: true, pos_slacks: 0, neg_slacks: 0}
    }

    pub fn from_string(vec_rat: Vec<Rational>, leq: bool, last_digit_negative: bool) -> Constraint {
        let mut vals: Vec<Rational> = Vec::new();
        if leq && last_digit_negative {
            vals = Constraint::negate_all(vec_rat);
            Constraint {constraint: vals, leq: false, pos_slacks: 1, neg_slacks: 1}
        } else if leq && !last_digit_negative {
            Constraint {constraint: vec_rat, leq: true, pos_slacks: 1, neg_slacks: 0}
        } else if !leq && last_digit_negative {
            vals = Constraint::negate_all(vec_rat);
            Constraint {constraint: vals, leq: true, pos_slacks: 1, neg_slacks: 0}
        } else {
            Constraint {constraint: vec_rat, leq: true, pos_slacks: 1, neg_slacks: 1}
        }
    }

    pub fn negate_all(vec_rat: Vec<Rational>) -> Vec<Rational> {
        vec_rat.into_iter().map(|x| -1 * x).collect::<Vec<Rational>>()
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.constraint.iter().fold(Ok(()), |result, rat| {
            result.and_then(|_| write!(f, "| {} |", rat))
        })
    }
}

impl EquationStringProcessable<Constraint> for Constraint {
    fn process(string: Vec<String>) -> Constraint {
        let mut constraint: Vec<Rational> = Vec::new();
        let mut leq: bool = false;
        let mut flag: bool = false;
        let mut last_digit_negative: bool = false;
        for name in string.iter() {
            match name.as_str() {
                "e" => continue,
                "geq" => {
                    leq = false;
                    flag = true;

                },
                "leq" => {
                    leq = true;
                    flag = true;
                }
                _ => {
                    let mut last_digit : Rational = Rational::from_string((*name).clone());
                    if flag {
                        last_digit_negative = last_digit < Rational::new(0, 1).unwrap();
                    }
                    constraint.push(last_digit);
                }
            }
        }
        Constraint::from_string(constraint, leq, last_digit_negative)
    }
}

