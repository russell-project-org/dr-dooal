use std::fmt;
use super::dooal_utils::Math;

#[derive(Debug)]
pub struct Rational {
    pub numerator: i32,
    pub denominator: i32
}

impl Rational {

    pub fn new(num: i32, denom: i32) -> Result<Rational,&'static str> {
        if denom == 0 {
            return Err("Invalid Rational Number as Denominator is Zero!");
        }

        let g : i32 = Math::gcd(num, denom);
        let numerator: i32 = num / g;
        let denominator: i32 = denom / g;
        Ok(Rational{numerator, denominator})
    }


}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}