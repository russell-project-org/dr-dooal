use crate::solver::tableau::Tableau;
pub struct Parser {

}

impl Parser {

    // temperorarily builds formula, constraint and variables
    // in the future, it will build the tableau and pass to main.rs
    pub fn process(mut lines : Vec<String>) {

        let mut tableau : Tableau = Tableau::new();
        let mut variable_logged : bool = false;

        for line in lines {
            let c : Vec<String> = line.split_whitespace().collect::<Vec<_>>().into_iter().map(String::from).collect();
            match c[0].as_str() {
                "f" => break,
                "e" => {
                    tableau.insert_constraint(c);
                    assert!(variable_logged);
                },
                "o" => {
                    tableau.insert_objective_function(c);
                    assert!(variable_logged);
                },
                "v" => {
                    tableau.insert_variables(c);
                    assert!(!variable_logged);
                    variable_logged = true;
                },
                _ => println!("comment or invalid") // delete later
            }

        }
        println!("CONSTRAINT COUNT : {:?}", tableau.constraints.len());
    }

}