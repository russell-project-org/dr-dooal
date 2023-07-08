use crate::solver::tableau::Tableau;
pub struct Parser {

}

impl Parser {

    // temperorarily builds formula, constraint and variables
    // in the future, it will build the tableau and pass to main.rs
    pub fn process(mut lines : Vec<String>) {

        let mut tableau : Tableau = Tableau::new();

        for line in lines {
            let c : Vec<String> = line.split_whitespace().collect::<Vec<_>>().into_iter().map(String::from).collect();
            match c[0].as_str() {
                "f" => {
                    println!("Terminate!");
                    break;
                },
                "e" => {
                    println!("Constraint found!");
                    tableau.insert_constraint(c);
                },
                "o" => {
                    println!("Objective found!");
                    tableau.insert_objective_function(c);
                },
                "v" => {
                    println!("variable found!");
                },
                _ => println!("comment or invalid")
            }
            
            // if !c[0].eq("c") {
            //     println!("{:?}", c);
            // }
        }
    }

}