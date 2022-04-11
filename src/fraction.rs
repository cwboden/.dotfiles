use std::fmt::{self, Display, Write};
use std::ops::{Add, Div, Mul};

use crate::math::{GreatestCommonDenominator, LeastCommonMultiple};

pub trait Integer<T>:
    Sized
    + Copy
    + Add<Output = T>
    + Display
    + Mul<Output = T>
    + LeastCommonMultiple
    + GreatestCommonDenominator
    + Div<Output = T>
    + Add<Output = T>
    + LeastCommonMultiple
    + GreatestCommonDenominator
{
}

impl Integer<u32> for u32 {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Fraction<T> {
    numerator: T,
    denominator: T,
}

impl<T> Fraction<T>
where
    T: Integer<T>,
{
    pub fn new(numerator: T, denominator: T) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        let lcm = self.denominator.lcm(&other.denominator);

        let first = (self.numerator * lcm) / other.denominator;
        let second = (other.numerator * lcm) / self.denominator;

        Self {
            numerator: first + second,
            denominator: lcm,
        }
    }
}

impl<T> Display for Fraction<T>
where
    T: Integer<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_assigns_correctly() {
        let fraction = Fraction::new(4u32, 6u32);

        assert_eq!(fraction.numerator, 4u32);
        assert_eq!(fraction.denominator, 6u32);
    }

    #[test]
    fn add_same_denominators() {
        let first = Fraction::new(4u32, 6u32);
        let second = Fraction::new(1u32, 6u32);

        let sum = first.add(&second);
        println!("{:?}", sum);

        assert_eq!(sum.numerator, 5u32);
        assert_eq!(sum.denominator, 6u32);
    }

    #[test]
    fn add_different_denominators() {
        let first = Fraction::new(1u32, 3u32);
        let second = Fraction::new(1u32, 2u32);

        let sum = first.add(&second);

        assert_eq!(sum.numerator, 5u32);
        assert_eq!(sum.denominator, 6u32);
    }

    #[test]
    fn display_formats_as_expected() {
        let fraction = Fraction::new(4u32, 7u32);

        let mut as_string = String::new();
        write!(&mut as_string, "{}", fraction).unwrap();

        assert_eq!(as_string, "4/7");
    }
}
