pub struct Pile<T> {
    cards: Vec<T>,
}

impl<T> Default for Pile<T> {
    fn default() -> Self {
        Self { cards: Vec::new() }
    }
}

impl<T: Clone> Pile<T> {
    #[must_use]
    pub fn new(cards: &[T]) -> Self {
        Self {
            cards: cards.to_vec(),
        }
    }

    pub fn peek(&self) -> Option<T> {
        self.cards.last().cloned()
    }

    pub fn add(&mut self, card: T) {
        self.cards.push(card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::TestCard;

    #[test]
    fn pile_peek_returns_top_card() {
        let pile = Pile::new(&[TestCard::X]);
        assert_eq!(pile.peek(), Some(TestCard::X));
    }

    #[test]
    fn pile_peek_if_empty() {
        assert_eq!(Pile::<()>::default().peek(), None);
    }

    #[test]
    fn pile_peek_added_card() {
        let mut pile = Pile::new(&[TestCard::X]);
        pile.add(TestCard::Y);

        assert_eq!(pile.peek(), Some(TestCard::Y));
    }
}
