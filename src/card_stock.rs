#![allow(dead_code)]

use core::fmt;

use crate::{
    card_stock_trait::ICardStock,
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

impl fmt::Display for CardStock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<CardStock>")?;
        writeln!(f, "{}", self.deck)?;
        writeln!(f, "</CardStock>")
    }
}

impl CardStock {
    pub fn new(deck: CardDeck) -> Self {
        Self { deck }
    }
}

impl ICardStock for CardStock {
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

    fn take_initial_cards(&mut self) -> InitialCards {
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
