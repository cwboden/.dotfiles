use std::sync::Mutex;

use crate::engine::types::Suit;

#[derive(Default)]
pub struct Favor {
    suit: Option<Suit>,
}

impl Favor {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn new_with_suit(suit: Suit) -> Self {
        Self { suit: Some(suit) }
    }
}

pub struct Bank {
    suit: Suit,
    quantity: Mutex<u8>,
}

impl Bank {
    pub fn new(suit: Suit, quantity: u8) -> Self {
        Self {
            suit,
            quantity: Mutex::new(quantity),
        }
    }

    pub fn get_quantity(&self) -> u8 {
        *self.quantity.lock().unwrap()
    }

    pub fn take_one(&self) -> Option<Favor> {
        let mut quantity = self.quantity.lock().unwrap();
        if *quantity > 0 {
            *quantity -= 1;
            Some(Favor::new_with_suit(self.suit))
        } else {
            None
        }
    }

    pub fn put_one(&self) {
        *self.quantity.lock().unwrap() += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_get_quantity() {
        let bank = Bank::new(Suit::Arcane, 4 /* quantity */);

        assert_eq!(bank.get_quantity(), 4);
    }

    #[test]
    fn bank_take_one_matches_suit() {
        let bank = Bank::new(Suit::Arcane, 1 /* quantity */);
        let favor = bank.take_one().unwrap();

        assert_eq!(favor.suit, Some(Suit::Arcane));
    }

    #[test]
    fn bank_take_one_affects_quantity() {
        let bank = Bank::new(Suit::Arcane, 4 /* quantity */);
        bank.take_one().unwrap();
        bank.take_one().unwrap();

        assert_eq!(bank.get_quantity(), 2);
    }

    #[test]
    fn bank_take_one_with_no_quantity_returns_none() {
        let bank = Bank::new(Suit::Arcane, 0 /* quantity */);

        assert!(bank.take_one().is_none());
    }

    #[test]
    fn bank_put_one_affects_quantity() {
        let bank = Bank::new(Suit::Arcane, 0 /* quantity */);
        bank.put_one();

        assert_eq!(bank.get_quantity(), 1);
    }
}
