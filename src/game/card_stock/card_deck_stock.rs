#![allow(dead_code)]

use core::fmt;

use crate::game::{
    card_deck::CardDeck,
    core::{Card, GameMode, PILES_AMOUNT},
};

use super::core::ICardStock;

pub struct CardDeckStock {
    deck: CardDeck,
}

impl fmt::Display for CardDeckStock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<CardStock>")?;
        writeln!(f, "{}", self.deck)?;
        writeln!(f, "</CardStock>")
    }
}

impl CardDeckStock {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            deck: CardDeck::new(game_mode),
        }
    }
}

impl ICardStock for CardDeckStock {
    fn deals_left(&self) -> usize {
        self.deck.len() / PILES_AMOUNT
    }

    fn take_deal(&mut self) -> Option<Vec<Card>> {
        if self.deals_left() > 0 {
            Some(self.deck.take_and_open_cards(PILES_AMOUNT))
        } else {
            None
        }
    }

    fn take_initial_cards(&mut self) -> Vec<Card> {
        if !self.deck.is_fresh() {
            panic!("We need fresh deck for initial cards dealership!")
        }

        let face_down_cards = self.deck.take_cards(44); // 4x5 + 6x4
        let face_up_cards = self.deck.take_and_open_cards(PILES_AMOUNT);

        face_down_cards
            .iter()
            .chain(face_up_cards.iter())
            .cloned()
            .collect()
    }
}
