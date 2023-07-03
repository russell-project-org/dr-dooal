
pub struct Parser {

}

impl Parser {

    // temperorarily builds formula, constraint and variables
    // in the future, it will build the tableau and pass to main.rs
    pub fn process(mut lines : Vec<String>) {
        for line in lines {
            let c : Vec<_> = line.split_whitespace().collect();
            
            if (!c[0].eq("c")) {
                println!("{:?}", c);
            }
        }

        // lines.into_iter()
        // .map(|x| x.split_whitespace().collect())
        // .for_each(|y : Vec<_>| println!("{:?}", y));
    }

    // fn split_string(line: String) -> Vec<&'static str>{
    //     line.split_whitespace().collect::<Vec<_>>()
    // }

}