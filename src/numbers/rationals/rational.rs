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
        let pair : (i32, i32) = (num / g, denom / g);
        // dbg!(pair);
        match pair {
            (0, _) => Ok(Rational { numerator:0, denominator: 1 }),
            (x, 1) => Ok(Rational { numerator: x, denominator: 1 }),
            (x, y) => {
                if (x > 0 && y < 0) || (x < 0 && y < 0) {
                    Ok(Rational { numerator: -1 * x, denominator: -1 * y })
                } else {
                    Ok(Rational { numerator: x, denominator: y })
                }
            }
        }
        
    }


}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tup: (i32, i32) = (self.numerator.clone(), self.denominator.clone());
        match tup {
            (0, _) => write!(f, "0"),
            (x, 1) => write!(f, "{}", x),
            (x, y) => write!(f, "{}/{}", x, y)
        }  
    }
}