use std::sync::Mutex;

use crate::engine::types::Color;

pub struct Warband<'a> {
    color: Color,
    debt: Debt<'a>,
}

impl<'a> Warband<'a> {
    fn new(color: Color, debt: Debt<'a>) -> Self {
        Self { color, debt }
    }
}

pub struct Bank {
    color: Color,
    quantity: Mutex<u8>,
}

/// Used to track outstanding debt on Warband pieces that return to the same
/// origin.
pub struct Debt<'a> {
    origin: &'a Bank,
}

impl Drop for Debt<'_> {
    fn drop(&mut self) {
        self.origin.put_one();
    }
}

impl Bank {
    pub fn new(color: Color, quantity: u8) -> Self {
        Self {
            color,
            quantity: Mutex::new(quantity),
        }
    }

    pub fn take_one_as_debt(&self) -> Option<Debt<'_>> {
        let mut quantity = self.quantity.lock().unwrap();
        if *quantity > 0 {
            *quantity -= 1;
            Some(Debt { origin: self })
        } else {
            None
        }
    }

    pub fn get_quantity(&self) -> u8 {
        *self.quantity.lock().unwrap()
    }

    pub fn take_one(&self) -> Option<Warband<'_>> {
        self.take_one_as_debt()
            .map(|debt| Warband::new(self.color.clone(), debt))
    }

    fn put_one(&self) {
        *self.quantity.lock().unwrap() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_get_quantity() {
        let bank = Bank::new(Color::Purple, 4 /* quantity */);

        assert_eq!(bank.get_quantity(), 4);
    }

    #[test]
    fn bank_take_one_matches_color() {
        let bank = Bank::new(Color::Purple, 1 /* quantity */);
        let warband = bank.take_one().unwrap();

        assert_eq!(warband.color, Color::Purple);
    }

    #[test]
    fn bank_take_one_affects_quantity() {
        let bank = Bank::new(Color::Purple, 4 /* quantity */);
        let _debt1 = bank.take_one().unwrap();
        let _debt2 = bank.take_one().unwrap();

        assert_eq!(bank.get_quantity(), 2);
    }

    #[test]
    fn bank_take_one_with_no_quantity_returns_none() {
        let bank = Bank::new(Color::Purple, 0 /* quantity */);

        assert!(bank.take_one().is_none());
    }

    #[test]
    fn bank_dropped_warband_returns_debt() {
        let bank = Bank::new(Color::Purple, 1 /* quantity */);

        {
            let _warband = bank.take_one();
        }

        assert_eq!(bank.get_quantity(), 1);
    }
}
