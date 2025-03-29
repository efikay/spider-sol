#![allow(dead_code)]

use core::fmt;

use crate::{
    available_move::AvailableMove, card_deck::CardDeck, card_sequence::CardSequence,
    card_stock::CardStock, game_tableau::GameTableau,
};

pub struct Game {
    tableau: GameTableau,
    stock: CardStock,
    complete_sequences: Vec<CardSequence>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<Game>")?;
        writeln!(f, "{}", self.stock)?;
        writeln!(f, "{}", self.tableau)?;
        writeln!(f, "Deals left: {}", self.stock.deals_left())?;
        writeln!(f, "Available moves:")?;
        for available_move in self.get_available_moves() {
            writeln!(f, "\t{}", available_move)?;
        }
        writeln!(f, "</Game>")
    }
}

impl Game {
    pub fn new(deck: CardDeck) -> Self {
        let mut stock = CardStock::new(deck);
        let tableau = GameTableau::new(stock.take_initial_cards());

        Self {
            stock,
            tableau,
            complete_sequences: vec![],
        }
    }

    pub fn deals_left(&self) -> usize {
        self.stock.deals_left()
    }

    pub fn deal_cards(&mut self) {
        if let Some(cards) = self.stock.take_deal() {
            self.tableau.take_deal(cards);
        } else {
            panic!("No deals left :(");
        }
    }

    pub fn get_available_moves(&self) -> Vec<AvailableMove> {
        self.tableau.calculate_available_moves()
    }

    pub fn search_and_update_complete_sequences(&mut self) {
        self.complete_sequences
            .extend(self.tableau.extract_complete_sequences());
    }

    pub fn is_won(&self) -> bool {
        self.complete_sequences.len() == 8
    }

    // TODO: More game logic
}
