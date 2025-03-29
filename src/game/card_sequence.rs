#![allow(dead_code)]

/** Note about this module.
 *
 * Debatable. Maybe I'll refuse to use this in future.
 * Looks-like overhead maybe.
 */
use crate::game::core::{Card, FULL_SEQUENCE_LENGTH, Suit};

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
    pub fn from_card(card: Card) -> Self {
        Self { cards: vec![card] }
    }

    pub fn add_cards(&mut self, cards: Vec<Card>) -> bool {
        let mut next_state = Vec::from(&self.cards[..]);
        next_state.extend(cards.iter());

        if is_valid_sequence(&next_state) {
            self.cards = next_state;

            return true;
        }
        false
    }
    pub fn add_card(&mut self, card: Card) -> bool {
        self.add_cards(vec![card])
    }

    pub fn is_complete(&self) -> bool {
        self.cards.len() == FULL_SEQUENCE_LENGTH
    }

    pub fn suit(&self) -> Suit {
        match self.cards.first() {
            Some(first_card) => first_card.suit,
            None => panic!("Sequence has to contain at least one card!"),
        }
    }

    pub fn group_into_sequences(cards: &mut Vec<Card>) -> Vec<CardSequence> {
        let mut sequences: Vec<CardSequence> = vec![];
        let mut pending_seq_cards: Vec<Card> = vec![];

        while !cards.is_empty() {
            let card = cards.remove(0);

            if pending_seq_cards.is_empty() {
                pending_seq_cards.push(card);
            } else if card.can_stack_on(pending_seq_cards.last().unwrap()) {
                pending_seq_cards.push(card);
            } else {
                sequences.push(CardSequence::new(pending_seq_cards));
                pending_seq_cards = vec![];
            }
        }

        sequences
    }
}

fn is_valid_sequence(cards: &Vec<Card>) -> bool {
    fn is_same_suit(cards: &Vec<Card>) -> bool {
        let suit = cards.first().unwrap().suit;

        cards.iter().all(|c| c.suit == suit)
    }

    fn is_valid_order(cards: &Vec<Card>) -> bool {
        cards.windows(2).all(|w| w[0].can_stack_on(&w[1]))
    }

    !cards.is_empty() && is_same_suit(&cards) && is_valid_order(&cards)
}

#[cfg(test)]
mod tests {
    use crate::game::core::Rank;

    use super::*;

    #[test]
    #[should_panic(expected = "Card sequence must be valid(same suit, valid order, not empty)!")]
    fn test_empty_sequence() {
        let no_cards: Vec<Card> = vec![];

        CardSequence::new(no_cards);
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
