use std::ops::{Add, Mul, Div};
use crate::math::{LeastCommonMultiple, GreatestCommonDenominator};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Fraction<T> where T: Add<Output = T> + LeastCommonMultiple + GreatestCommonDenominator {
    numerator: T,
    denominator: T,
}

impl<T> Fraction<T> where T: Copy + Add<Output = T> + Mul<Output = T> + LeastCommonMultiple + GreatestCommonDenominator + Div<Output = T> {
    fn new(numerator: T, denominator: T) -> Self {
        return Self { numerator, denominator }
    }

    fn add(&self, other: &Self) -> Self {

        let gcd = self.denominator.gcd(&other.denominator);
        let lcm = self.denominator.lcm(&other.denominator);

        let first = (self.numerator * lcm) / other.denominator;
        let second = (other.numerator * lcm) / self.denominator;

        Self {
            numerator: first + second,
            denominator: lcm,
        }
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
    fn test_add_same_denominators() {
        let first = Fraction::new(4u32, 6u32);
        let second = Fraction::new(1u32, 6u32);

        let sum = first.add(&second);
        println!("{:?}", sum);

        assert_eq!(sum.numerator, 5u32);
        assert_eq!(sum.denominator, 6u32);
    }

    #[test]
    fn test_add_different_denominators() {
        let first = Fraction::new(1u32, 3u32);
        let second = Fraction::new(1u32, 2u32);

        let sum = first.add(&second);

        assert_eq!(sum.numerator, 5u32);
        assert_eq!(sum.denominator, 6u32);
    }
}
