use crate::Rational;

pub struct Constraint {
    pub row : Vec<Rational>
}

impl Constraint {
    pub fn new(vec_num: Vec<String>) -> Constraint {
        let mut row : Vec<Rational> = Vec::new();
        Constraint {row : row}
    }
}

