use crate::Rational;

pub struct Variable {
    pub is_slack : bool,
    pub id : i32,
    pub is_basic : bool,
    pub var_value: Option<Rational>
}

impl Variable {
    pub fn new(id: i32, is_slack: bool, is_basic: bool) -> Variable {
        Variable {is_slack: is_slack, id: id, is_basic: is_basic, var_value: None }
    }

    pub fn assign_value(&mut self, value: Rational) {
        self.var_value = Some(value);
    }
}