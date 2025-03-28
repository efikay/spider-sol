#![allow(dead_code)]

use crate::{core::Card, data_structures::Stack};

pub struct CardPile {
    cards: Stack<Card>,
}

impl CardPile {
    pub fn new() -> Self {
        Self {
            cards: Stack::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
