use std::fmt;
use std::fmt::Formatter;
use crate::equations::EquationStringProcessable;
use crate::Rational;

pub struct Objective {
    pub objective_function : Vec<Rational>
}

impl Objective {

    pub fn new() -> Objective {
        Objective {objective_function: Vec::new()}
    }

}

impl EquationStringProcessable<Objective> for Objective {
    fn process(string: Vec<String>) -> Objective {
        let mut obj_fn : Vec<Rational> = Vec::new();
        for name in string.iter() {
            match name.as_str() {
                "o" => continue,
                _ => obj_fn.push(Rational::from_string((*name).clone()))
            }
        }
        Objective {objective_function: obj_fn}
    }
}

impl fmt::Display for Objective {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.objective_function.iter().fold(Ok(()), |result, rat| {
            result.and_then(|_| write!(f, "| {} |", rat))
        })
    }
}


