struct Deck<T> {
    cards: Vec<T>,
}

impl<T: Clone> Deck<T> {
    pub fn new(cards: &[T]) -> Self {
        Self {
            cards: cards.to_vec(),
        }
    }
}

impl<T> IntoIterator for Deck<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestCard {
        X,
        Y,
        Z,
    }

    #[test]
    fn deck_into_iterator() {
        let cards = vec![TestCard::X, TestCard::Y, TestCard::Z];
        let deck = Deck::new(&cards);

        assert_eq!(cards, deck.into_iter().collect::<Vec<TestCard>>());
    }
}
