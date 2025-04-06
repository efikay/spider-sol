#![allow(dead_code)]

use core::fmt;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    data_structures::Stack,
    game::core::{Card, GameMode, Suit},
};

const FRESH_DECK_CARDS_AMOUNT: usize = 104;

pub struct CardDeck {
    remaining_cards: Stack<Card>,
}

impl CardDeck {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            remaining_cards: Stack::from_iter(make_shuffled_game_mode_deck(game_mode)),
        }
    }

    pub fn len(&self) -> usize {
        self.remaining_cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.remaining_cards.is_empty()
    }

    pub fn is_fresh(&self) -> bool {
        self.len() == FRESH_DECK_CARDS_AMOUNT
    }

    pub fn take_card(&mut self) -> Option<Card> {
        self.remaining_cards.pop()
    }

    pub fn take_cards(&mut self, desired_amount: usize) -> Vec<Card> {
        self.remaining_cards.pop_many(desired_amount)
    }

    pub fn take_and_open_cards(&mut self, desired_amount: usize) -> Vec<Card> {
        let mut cards = self.take_cards(desired_amount);
        for card in &mut cards {
            card.is_opened = true;
        }

        cards
    }
}

/// ------- Formatting ------- ///
impl fmt::Display for CardDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.remaining_cards)
    }
}

/// ------- Helpers ------- ///
fn make_shuffled_game_mode_deck(game_mode: GameMode) -> Vec<Card> {
    fn shuffle_cards(cards: &mut Vec<Card>) {
        let mut rng = rand::rng();

        cards.shuffle(&mut rng);
    }

    let complete_seqs_per_suit = match game_mode {
        GameMode::OneSuit => 8,
        GameMode::TwoSuits => 4,
        GameMode::FourSuits => 2,
    };
    let suits = match game_mode {
        GameMode::OneSuit => vec![Suit::Diamonds],
        GameMode::TwoSuits => vec![Suit::Spades, Suit::Hearts],
        GameMode::FourSuits => Suit::iter().collect(),
    };

    let mut cards = suits
        .iter()
        .map(|suit| {
            let complete_seq = Card::make_complete_sequence_of(*suit);

            std::iter::repeat(complete_seq)
                .take(complete_seqs_per_suit)
                .flatten()
        })
        .flatten()
        .collect();
    shuffle_cards(&mut cards);

    cards
}
