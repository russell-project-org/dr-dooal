use crate::Rational;

pub struct Objective {
    pub objective_function : Vec<Rational>
}

impl Objective {

    pub fn new() -> Objective {
        Objective {objective_function: Vec::new()}
    }
}

