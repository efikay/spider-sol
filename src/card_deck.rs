#![allow(dead_code)]
use core::fmt;

use rand::prelude::*;

use strum::IntoEnumIterator;

use crate::{
    core::{Card, GameMode, Suit},
    data_structures::Stack,
};

pub struct CardDeck {
    remaining_cards: Stack<Card>,
}

impl fmt::Display for CardDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.remaining_cards)
    }
}

impl CardDeck {
    pub fn new(game_mode: GameMode) -> Self {
        let full_sequences_per_suit = match game_mode {
            GameMode::OneSuit => 8,
            GameMode::TwoSuits => 4,
            GameMode::FourSuits => 2,
        };
        let suits = match game_mode {
            GameMode::OneSuit => vec![Suit::Spades],
            GameMode::TwoSuits => vec![Suit::Spades, Suit::Hearts],
            GameMode::FourSuits => Suit::iter().collect(),
        };

        let mut full_deck = make_deck_of(suits, full_sequences_per_suit);
        shuffle_cards(&mut full_deck);

        Self {
            remaining_cards: Stack::from_iter(full_deck),
        }
    }

    pub fn len(&self) -> usize {
        self.remaining_cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn take_card(&mut self) -> Option<Card> {
        self.remaining_cards.pop()
    }
}

fn shuffle_cards(cards: &mut Vec<Card>) {
    let mut rng = rand::rng();

    cards.shuffle(&mut rng);
}

fn make_deck_of(suits: Vec<Suit>, full_sequences_per_suit: usize) -> Vec<Card> {
    suits
        .iter()
        .map(|suit| {
            let full_sequence = Card::make_full_sequence_of(*suit);

            std::iter::repeat(full_sequence)
                .take(full_sequences_per_suit)
                .flatten()
        })
        .flatten()
        .collect()
}
