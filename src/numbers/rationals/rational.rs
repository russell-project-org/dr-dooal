use std::fmt;
use std::ops:: {Add, Sub, Mul, Div};
use std::cmp:: {PartialEq, Eq};
use super::dooal_utils::Math;

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.numerator * other.denominator == self.denominator * other.numerator
    }
}
 
impl Eq for Rational {}

impl Add for Rational {
    type Output = Rational;
    fn add(self, other: Rational) -> Rational {
        Rational::new(self.numerator * other.denominator + self.denominator * other.numerator,
            self.denominator * other.denominator).unwrap()
    }
}

impl Add<i32> for Rational {
    type Output = Rational;
    fn add(self, other: i32) -> Rational {
        Rational::add(self, Rational{numerator: other, denominator: 1})
    }
}

impl Add<Rational> for i32 {
    type Output = Rational;
    fn add(self, rhs: Rational) -> Rational {
        Rational::add(Rational{numerator: self, denominator: 1}, rhs)
    }
}

impl Sub for Rational {
    type Output = Rational;
    fn sub(self, other: Rational) -> Rational {
        Rational::new(self.numerator * other.denominator - self.denominator * other.numerator,
            self.denominator * other.denominator).unwrap()
    }
}

impl Sub<Rational> for i32 {
    type Output = Rational;
    fn sub(self, rhs: Rational) -> Rational {
        Rational::sub(Rational { numerator: self, denominator: 1 }, rhs)
    }
}

impl Sub<i32> for Rational {
    type Output = Rational;
    fn sub(self, other: i32) -> Rational {
        Rational::sub(self, Rational {numerator: other, denominator: 1})
    }
}

impl Mul for Rational {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.numerator * other.numerator,
            self.denominator * other.denominator).unwrap()
    }
}

impl Mul<i32> for Rational {
    type Output = Rational;
    fn mul(self, other: i32) -> Rational {
        Rational::mul(self, Rational{numerator: other, denominator: 1})
    }
}

impl Mul<Rational> for i32 {
    type Output = Rational;
    fn mul(self, other: Rational) -> Rational {
        Rational::mul(Rational { numerator: self, denominator: 1 }, other)
    }
}

impl Div for Rational {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        Rational::new(self.numerator * other.denominator,
            self.denominator * other.numerator).unwrap()
    }
}

impl Div<i32> for Rational {
    type Output = Rational;
    fn div(self, other: i32) -> Rational {
        Rational::div(self, Rational {numerator: other, denominator: 1})
    }
}

impl Div<Rational> for i32 {
    type Output = Rational;
    fn div(self, other: Rational) -> Rational {
        Rational::div(Rational { numerator: self, denominator: 1 }, other)
    }
}