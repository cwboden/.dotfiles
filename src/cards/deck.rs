use rand::Rng;
use std::collections::VecDeque;

struct Deck<T> {
    in_cards: VecDeque<T>,
    out_cards: VecDeque<T>,
}

#[derive(Debug, Eq, PartialEq)]
enum Error {
    DeckEmpty,
}

type Result<T> = std::result::Result<T, Error>;

impl<T: Clone> Deck<T> {
    pub fn new(cards: &[T]) -> Self {
        Self {
            in_cards: cards.iter().map(|c| c.clone()).collect(),
            out_cards: VecDeque::new(),
        }
    }

    pub fn deal_one(&mut self) -> Result<T> {
        if let Some(card) = self.in_cards.pop_front() {
            self.out_cards.push_front(card.clone());
            Ok(card)
        } else {
            Err(Error::DeckEmpty)
        }
    }

    pub fn reset(&mut self) {
        self.out_cards
            .iter()
            .for_each(|c| self.in_cards.push_front(c.clone()));
        self.out_cards.clear();
    }

    pub fn shuffle(&mut self) {
        // XXX: Fairly naive implementation of the Fisher-Yates shuffle. There might be a better way
        // to do this!
        let mut rng = rand::thread_rng();
        let mut i = self.in_cards.len();
        while i >= 2 {
            i -= 1;
            self.in_cards.swap(i, rng.gen_range(0..i));
        }
    }
}

impl<T> IntoIterator for Deck<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.in_cards.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::test::TestCard;

    #[test]
    fn deck_into_iterator() {
        let cards = vec![TestCard::X, TestCard::Y, TestCard::Z];
        let deck = Deck::new(&cards);

        assert_eq!(cards, deck.into_iter().collect::<Vec<TestCard>>());
    }

    #[test]
    fn deck_deal_one() {
        let mut deck = Deck::new(&[TestCard::X]);

        assert_eq!(deck.deal_one().unwrap(), TestCard::X);
    }

    #[test]
    fn deck_deal_one_errors_when_empty() {
        let mut deck = Deck::new(&[TestCard::X]);

        deck.deal_one().unwrap();
        assert_eq!(deck.deal_one(), Err(Error::DeckEmpty));
    }

    #[test]
    fn deck_deal_one_multiple_times() {
        let mut deck = Deck::new(&[TestCard::X, TestCard::Y, TestCard::Z]);

        assert_eq!(deck.deal_one().unwrap(), TestCard::X);
        assert_eq!(deck.deal_one().unwrap(), TestCard::Y);
        assert_eq!(deck.deal_one().unwrap(), TestCard::Z);
    }

    #[test]
    fn deck_reset_adds_cards_back_to_deck() {
        let mut deck = Deck::new(&[TestCard::X]);

        deck.deal_one().unwrap();

        deck.reset();
        assert_eq!(deck.deal_one().unwrap(), TestCard::X);
    }

    #[test]
    fn deck_reset_preserves_order() {
        let mut deck = Deck::new(&[TestCard::X, TestCard::Y, TestCard::Z]);

        deck.deal_one().unwrap();
        deck.deal_one().unwrap();

        deck.reset();

        assert_eq!(deck.deal_one().unwrap(), TestCard::X);
        assert_eq!(deck.deal_one().unwrap(), TestCard::Y);
        assert_eq!(deck.deal_one().unwrap(), TestCard::Z);
    }
}
