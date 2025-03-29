#![allow(dead_code)]

use crate::{
    card_deck::CardDeck,
    core::{Card, PILES_AMOUNT},
};
pub struct CardStock {
    deck: CardDeck,
}

pub struct InitialCards {
    pub face_down_cards: Vec<Card>,
    pub face_up_cards: Vec<Card>,
}

impl CardStock {
    pub fn new(deck: CardDeck) -> Self {
        Self { deck }
    }

    pub fn deals_left(&self) -> usize {
        self.deck.len() / PILES_AMOUNT
    }

    pub fn take_initial_cards(&mut self) -> InitialCards {
        if !self.deck.is_fresh() {
            panic!("We need fresh deck for initial cards allocation!")
        }

        let face_down_cards = self.deck.take_cards(44); // 4x5 + 6x4
        let face_up_cards = self.deck.take_and_open_cards(PILES_AMOUNT);

        InitialCards {
            face_down_cards,
            face_up_cards,
        }
    }
}
