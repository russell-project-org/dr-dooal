use crate::Rational;

pub struct Variable {
    pub is_slack : bool,
    pub id : i32,
    pub var_value: Option<Rational>
}

impl Variable {
    pub fn new(id: int32, is_slack: bool) -> Variable {
        Variable(id: id, is_slack: is_slack, None)
    }

    pub fn assign_value(&mut self, value: Rational) {
        self.var_value = Some(value);
    }
}