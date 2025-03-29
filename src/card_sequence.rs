#![allow(dead_code)]

/** Note about this module.
 *
 * Debatable. Maybe I'll refuse to use this in future.
 * Looks-like overhead maybe.
 */
use crate::{core::Card, core::Suit};

#[derive(Debug, Clone)]
pub struct CardSequence {
    pub cards: Vec<Card>,
}

impl CardSequence {
    pub fn new(cards: Vec<Card>) -> Self {
        if !is_valid_sequence(&cards) {
            panic!("Card sequence must be valid(same suit, valid order, not empty)!");
        }

        Self { cards }
    }

    pub fn suit(&self) -> Suit {
        match self.cards.first() {
            Some(first_card) => first_card.suit,
            None => panic!("Sequence have to contain at least one card!"),
        }
    }
}

fn is_valid_sequence(cards: &Vec<Card>) -> bool {
    !cards.is_empty() && is_same_suit(&cards) && is_valid_order(&cards)
}

fn is_same_suit(cards: &Vec<Card>) -> bool {
    let suit = cards.first().unwrap().suit;

    cards.iter().all(|c| c.suit == suit)
}

fn is_valid_order(cards: &Vec<Card>) -> bool {
    cards.windows(2).all(|w| w[0].can_stack_on(&w[1]))
}

#[cfg(test)]
mod tests {
    use crate::core::Rank;

    use super::*;

    #[test]
    #[should_panic(expected = "Card sequence must be valid(same suit, valid order, not empty)!")]
    fn test_empty_sequence() {
        let no_cards: Vec<Card> = vec![];

        CardSequence::new(no_cards);
    }

    #[test]
    fn test_order_validation() {
        let valid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Four, Suit::Clubs),
            ],
            vec![
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::King, Suit::Clubs),
            ],
            vec![Card::new(Rank::Ten, Suit::Clubs)],
        ];
        for cards in valid_sequences {
            assert_eq!(is_valid_order(&cards), true);
        }

        let invalid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Four, Suit::Clubs),
            ],
            vec![
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Clubs),
            ],
            vec![
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Clubs),
            ],
        ];
        for cards in invalid_sequences {
            assert_eq!(is_valid_order(&cards), false);
        }
    }

    #[test]
    fn test_suit_validation() {
        let valid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Hearts),
                Card::new(Rank::Ace, Suit::Hearts),
            ],
            vec![Card::new(Rank::Ace, Suit::Diamonds)],
            vec![
                Card::new(Rank::Ten, Suit::Spades),
                Card::new(Rank::Ten, Suit::Spades),
                Card::new(Rank::Ten, Suit::Spades),
            ],
        ];
        for cards in valid_sequences {
            assert_eq!(is_same_suit(&cards), true);
        }

        let invalid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Diamonds),
                Card::new(Rank::Ace, Suit::Clubs),
            ],
            vec![
                Card::new(Rank::King, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Diamonds),
            ],
            vec![
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Diamonds),
            ],
            vec![
                Card::new(Rank::Three, Suit::Diamonds),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Ace, Suit::Clubs),
            ],
        ];
        for cards in invalid_sequences {
            assert_eq!(is_same_suit(&cards), false);
        }
    }

    #[test]
    fn test_sequence_validation() {
        let valid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Two, Suit::Clubs),
                Card::new(Rank::Three, Suit::Clubs),
                Card::new(Rank::Four, Suit::Clubs),
            ],
            vec![
                Card::new(Rank::Queen, Suit::Clubs),
                Card::new(Rank::King, Suit::Clubs),
            ],
            vec![Card::new(Rank::Ten, Suit::Clubs)],
        ];
        for cards in valid_sequences {
            assert_eq!(is_valid_sequence(&cards), true);
        }

        let invalid_sequences = vec![
            vec![
                Card::new(Rank::Ace, Suit::Clubs),
                Card::new(Rank::Two, Suit::Diamonds),
                Card::new(Rank::Three, Suit::Clubs),
            ],
            vec![],
            vec![
                Card::new(Rank::Four, Suit::Clubs),
                Card::new(Rank::Five, Suit::Clubs),
                Card::new(Rank::Seven, Suit::Clubs),
            ],
        ];
        for cards in invalid_sequences {
            assert_eq!(is_valid_sequence(&cards), false);
        }
    }
}
