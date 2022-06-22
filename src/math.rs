pub trait GreatestCommonDenominator {
    #[must_use]
    fn gcd(&self, other: &Self) -> Self;
}

impl GreatestCommonDenominator for u32 {
    #[must_use]
    fn gcd(&self, other: &Self) -> Self {
        if self < other {
            other.gcd(self)
        } else if *other == 0u32 {
            *self
        } else {
            other.gcd(&(self % other))
        }
    }
}

pub trait LeastCommonMultiple {
    #[must_use]
    fn lcm(&self, other: &Self) -> Self;
}

impl LeastCommonMultiple for u32 {
    #[must_use]
    fn lcm(&self, other: &Self) -> Self {
        (self * other) / self.gcd(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_correct_for_u32() {
        assert_eq!(3u32.gcd(&6u32), 3u32);
        assert_eq!(8u32.gcd(&12u32), 4u32);
        assert_eq!(30u32.gcd(&40u32), 10u32);
        assert_eq!(285u32.gcd(&741u32), 57u32);
    }

    #[test]
    fn lcm_correct_for_u32() {
        assert_eq!(3u32.lcm(&6u32), 6u32);
        assert_eq!(8u32.lcm(&12u32), 24u32);
        assert_eq!(30u32.lcm(&40u32), 120u32);
        assert_eq!(285u32.lcm(&741u32), 3705u32);
    }
}
