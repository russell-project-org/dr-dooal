use crate::equations::constraint::Constraint;
use crate::equations::objective::Objective;
use crate::equations::variable::Variable;

pub struct Tableau {
    pub objective : Objective,
    pub constraints : Vec<Constraint>,
    pub vars : Vec<Variable>
}

impl Tableau {
    pub fn insert_constraint() {

    }

    pub fn insert_objective_function() {

    }

    pub fn insert_variable() {
        
    }
}