use crate::equations::constraint::Constraint;
use crate::equations::{EqualityDetector, EquationStringProcessable};
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
        match EqualityDetector::vec_contains_eq_keyword(&string) {
            true => {
                let mut constraints: Vec<Constraint> = Tableau::break_up_equality_constraint(&string);
                self.constraints.append(&mut constraints);
            },
            false => {
                let constraint : Constraint = Constraint::process(string);
                println!("{}", constraint);
                self.constraints.push(constraint);
            }
        }
    }

    pub fn map_replace(vector : &Vec<String>, old_val: String, new_val : String) -> Vec<String> {
        let mut new_vec : Vec<String> = Vec::new();
        let reference_string = &old_val.clone();
        for string in vector.iter() {
            if string.eq(reference_string) {
                 new_vec.push(new_val.clone());
            } else {
                new_vec.push(string.clone());
            }
        }
        new_vec
    }

    pub fn break_up_equality_constraint(vector : &Vec<String>) -> Vec<Constraint>{
        let leq_version : Constraint = Constraint::process(
            Tableau::map_replace(&vector, String::from("eq"),
                                 String::from("leq")));
        let geq_version : Constraint = Constraint::process(
            Tableau::map_replace(&vector, String::from("eq"),
                                 String::from("geq")));
        //println!("GEQ {:?}", geq_version);
        vec![leq_version, geq_version]
    }

    pub fn insert_objective_function(&mut self, string: Vec<String>) {
        self.objective = Objective::process(string);
        //println!("{}", self.objective);
    }

    pub fn insert_variables(&mut self, string: Vec<String>) {
        let mut integers : Vec<i32> = string.into_iter()
            .map(|x| x.parse().unwrap_or_else(|x| {
                println!("Not a valid input for integers in file!");
                std::process::exit(1);
            })).collect();
        let num_integers = integers.len();
        for i in 0..=num_integers - 1 {
            match integers[i] {
                -1 => self.vars.push(Variable::new()),
                1 => self.vars.push(Variable::new()),
                _ => {
                    println!("Invalid number detected! Aborting...");
                    std::process::exit(1);
                }
            }
        }
    }
}