use crate::solver::tableau::Tableau;
pub struct Parser {

}

impl Parser {

    // temperorarily builds formula, constraint and variables
    // in the future, it will build the tableau and pass to main.rs
    pub fn process(mut lines : Vec<String>) {

        let mut tableau : Tableau = Tableau{objective:crate::equations::objective::Objective {  },constraints:Vec::new(), vars:Vec::new()};

        for line in lines {
            let c : Vec<_> = line.split_whitespace().collect();

            match c[0] {
                "f" => {
                    println!("Terminate!");
                    break;
                },
                "e" => {
                    println!("Constraint found!");
                },
                "o" => {
                    println!("Objective found!");
                },
                "v" => {
                    println!("variable found!");
                },
                _ => println!("comment or invalid")
            }
            
            if !c[0].eq("c") {
                println!("{:?}", c);
            }
        }
    }

}