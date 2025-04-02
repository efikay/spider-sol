#![allow(dead_code)]

use std::fmt;

use crate::game::core::{COMPLETE_SEQUENCE_LENGTH, Card, PILES_AMOUNT};

use super::card_move::{CardMove, CardMoveBuilder};

const NO_CARDS: [Card; 0] = [];

pub struct CardPileV2 {
    // Desc order (K,Q,J,10,9..etc)
    cards: Vec<Card>,
    // Not rly good but super handy
    index: usize,
}

impl PartialEq for CardPileV2 {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl CardPileV2 {
    pub fn init_piles() -> [CardPileV2; PILES_AMOUNT] {
        std::array::from_fn(|index| CardPileV2::new(index))
    }
    pub fn new(index: usize) -> Self {
        Self {
            index,
            cards: vec![],
        }
    }
    pub fn from_cards(cards: Vec<Card>, index: usize) -> Self {
        Self { index, cards }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
    fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn add_start_card(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn add_deal_card(&mut self, card: Card) {
        if self.is_empty() {
            panic!("cannot add deal card on empty pile!");
        }

        self.cards.push(card);
    }

    pub fn try_extract_complete_sequence(&mut self) -> Option<[Card; COMPLETE_SEQUENCE_LENGTH]> {
        let cards = self.playable_cards();

        if cards.len() == COMPLETE_SEQUENCE_LENGTH {
            let full_seq_range = self.len() - (COMPLETE_SEQUENCE_LENGTH)..;
            let full_seq_cards = self.cards.drain(full_seq_range).collect::<Vec<Card>>();

            let std_cards: [Card; COMPLETE_SEQUENCE_LENGTH] = match full_seq_cards.try_into() {
                Ok(arr) => arr,
                Err(_) => panic!("Something is wrong :("),
            };

            Some(std_cards)
        } else {
            None
        }
    }

    pub fn perform_card_move(&mut self, card_move: &CardMove, target_pile: &CardPileV2) {
        todo!()
    }

    pub fn calc_moves_to(&self, other: &CardPileV2) -> Vec<CardMove> {
        let cards = self.playable_cards();
        let other_cards = other.playable_cards();

        if !self.can_move_to(other) {
            return vec![];
        }

        if other_cards.is_empty() {
            cards
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    CardMoveBuilder::from_pile(self.index)
                        .using_card(index)
                        .to_empty_pile(other.index)
                        .build()
                })
                .collect()
        } else {
            vec![
                CardMoveBuilder::from_pile(self.index)
                    .to_card_pile(other.index)
                    .build(),
            ]
        }
    }

    fn can_move_to(&self, other: &CardPileV2) -> bool {
        let cards = self.playable_cards();
        let other_cards = other.playable_cards();

        if cards.is_empty() {
            return false;
        }
        if other_cards.is_empty() {
            return true;
        }

        cards.iter().any(|card| {
            other_cards
                .iter()
                .any(|other_card| card.can_move_on(other_card))
        })
    }

    fn playable_cards(&self) -> &[Card] {
        let sequences = self.seqs();

        if let Some(last_seq) = sequences.last() {
            assert!(last_seq.iter().all(|c| c.is_opened));

            last_seq
        } else {
            &NO_CARDS
        }
    }

    fn seqs(&self) -> Vec<&[Card]> {
        if self.cards.is_empty() {
            return Vec::new();
        }

        let mut sequences = Vec::new();
        let mut seq_start = 0;

        for i in 1..self.cards.len() {
            let card = &self.cards[i];
            let prev_card = &self.cards[i - 1];

            if !card.can_stack_on(prev_card) {
                sequences.push(&self.cards[seq_start..i]);
                seq_start = i;
            }
        }
        sequences.push(&self.cards[seq_start..]);

        sequences
    }
}

impl fmt::Display for CardPileV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for seq in &self.seqs() {
            let cards_str = seq
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(",");

            write!(f, " [{}] ", cards_str)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::core::{Rank, Suit};

    use super::*;

    #[test]
    fn should_be_empty_initially() {
        let empty_pile = CardPileV2::new(0);

        assert!(empty_pile.is_empty());
    }

    #[test]
    fn should_correctly_extract_complete_sequence() {
        let mut pile = CardPileV2::from_cards(
            Card::make_complete_sequence_of_opened(Suit::Diamonds)
                .iter()
                .rev()
                .cloned()
                .collect(),
            0,
        );

        let expected_seq = [
            Card::new_opened(Rank::King, Suit::Diamonds),
            Card::new_opened(Rank::Queen, Suit::Diamonds),
            Card::new_opened(Rank::Jack, Suit::Diamonds),
            Card::new_opened(Rank::Ten, Suit::Diamonds),
            Card::new_opened(Rank::Nine, Suit::Diamonds),
            Card::new_opened(Rank::Eight, Suit::Diamonds),
            Card::new_opened(Rank::Seven, Suit::Diamonds),
            Card::new_opened(Rank::Six, Suit::Diamonds),
            Card::new_opened(Rank::Five, Suit::Diamonds),
            Card::new_opened(Rank::Four, Suit::Diamonds),
            Card::new_opened(Rank::Three, Suit::Diamonds),
            Card::new_opened(Rank::Two, Suit::Diamonds),
            Card::new_opened(Rank::Ace, Suit::Diamonds),
        ];

        assert_eq!(pile.len(), COMPLETE_SEQUENCE_LENGTH);
        assert_eq!(pile.seqs().len(), 1);

        assert_eq!(pile.try_extract_complete_sequence(), Some(expected_seq));
        assert!(pile.is_empty());
    }

    #[test]
    fn should_correctly_group_into_sequences() {
        let mut pile = CardPileV2::new(0);
        pile.add_start_card(Card::new(Rank::King, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Queen, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Jack, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Ten, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Nine, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Eight, Suit::Spades));
        //
        pile.add_start_card(Card::new(Rank::Eight, Suit::Spades));
        //
        pile.add_start_card(Card::new(Rank::Eight, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Seven, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Six, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Five, Suit::Spades));
        //
        pile.add_start_card(Card::new(Rank::Three, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Two, Suit::Spades));
        pile.add_start_card(Card::new(Rank::Ace, Suit::Spades));

        let expected_sequences: Vec<Vec<Card>> = vec![
            vec![
                Card::new(Rank::King, Suit::Spades),
                Card::new(Rank::Queen, Suit::Spades),
                Card::new(Rank::Jack, Suit::Spades),
                Card::new(Rank::Ten, Suit::Spades),
                Card::new(Rank::Nine, Suit::Spades),
                Card::new(Rank::Eight, Suit::Spades),
            ],
            vec![Card::new(Rank::Eight, Suit::Spades)],
            vec![
                Card::new(Rank::Eight, Suit::Spades),
                Card::new(Rank::Seven, Suit::Spades),
                Card::new(Rank::Six, Suit::Spades),
                Card::new(Rank::Five, Suit::Spades),
            ],
            vec![
                Card::new(Rank::Three, Suit::Spades),
                Card::new(Rank::Two, Suit::Spades),
                Card::new(Rank::Ace, Suit::Spades),
            ],
        ];

        let sequences: Vec<Vec<Card>> = pile
            .seqs()
            .iter()
            .map(|seq| {
                seq.iter()
                    .map(|card| Card::from(*card))
                    .collect::<Vec<Card>>()
            })
            .collect();

        assert_eq!(expected_sequences, sequences);
    }
}
