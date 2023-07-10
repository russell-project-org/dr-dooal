use crate::Rational;

pub struct Variable {
    pub is_y_slack : bool,
    pub is_s_slack : bool,
    pub id : i32,
    pub is_basic : bool,
    pub is_positive_slack : bool,
    pub var_value: Option<Rational>
}

impl Variable {
    pub fn new() -> Variable {
        Variable {is_y_slack: false, is_s_slack: false, id: -1,
            is_basic: false, is_positive_slack: false, var_value: None }
    }

    pub fn identify_s_slack(&mut self, is_s_slack: bool, is_positive: bool) {
        self.is_s_slack = is_s_slack;
        self.is_y_slack = !is_s_slack;
        self.is_positive_slack = is_positive;
    }

    pub fn identify_y_slack(&mut self, is_y_slack: bool, is_positive: bool) {
        self.is_y_slack = is_y_slack;
        self.is_s_slack = !is_y_slack;
        self.is_positive_slack = is_positive;
    }



    pub fn assign_value(&mut self, value: Rational) {
        self.var_value = Some(value);
    }
}