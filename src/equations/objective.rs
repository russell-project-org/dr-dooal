use crate::equations::EquationStringProcessable;
use crate::Rational;

pub struct Objective {
    pub objective_function : Vec<Rational>
}

impl Objective {

    pub fn new() -> Objective {
        Objective {objective_function: Vec::new()}
    }

    pub fn process(string: Vec<String>) -> Objective {
        let mut obj_fn : Vec<Rational> = Vec::new();
        // for name in string.iter() {
        //     *name =
        // }
        Objective::new()
    }
}


