use crate::equations::constraint::Constraint;
use crate::equations::EquationStringProcessable;
use crate::equations::objective::Objective;
use crate::equations::variable::Variable;

pub struct Tableau {
    pub objective : Objective,
    pub constraints : Vec<Constraint>,
    pub vars : Vec<Variable>
}

impl Tableau {

    pub fn new() -> Tableau {
        Tableau {objective: Objective::new(), constraints: Vec::new(), vars: Vec::new()}
    }

    pub fn insert_constraint(&mut self, string: Vec<String>) {
        let constraint : Constraint = Constraint::process(string);
        println!("{}", constraint);
        self.constraints.push(constraint);
    }

    pub fn insert_objective_function(&mut self, string: Vec<String>) {
        self.objective = Objective::process(string);
        println!("{}", self.objective);
    }

    pub fn insert_variable() {
        
    }
}