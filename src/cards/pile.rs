#[derive(Default)]
struct Pile<T> {
    cards: Vec<T>,
}

impl<T: Clone> Pile<T> {
    pub fn new(cards: &[T]) -> Self {
        Self {
            cards: cards.to_vec(),
        }
    }

    pub fn peek(&self) -> Option<T> {
        self.cards.last().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::test::TestCard;

    #[test]
    fn pile_peek_returns_top_card() {
        let pile = Pile::new(&[TestCard::X]);
        assert_eq!(pile.peek(), Some(TestCard::X));
    }

    #[test]
    fn pile_peek_if_empty() {
        assert_eq!(Pile::<()>::default().peek(), None);
    }
}
