#![allow(dead_code)]

use core::fmt;
use std::{cell::RefCell, rc::Rc};

use super::{
    card_stock::ICardStock,
    core::{COMPLETE_SEQUENCE_LENGTH, Card},
    v2::CardMove,
};
use crate::game::game_tableau::GameTableau;

const COMPLETE_SEQUENCES_TO_WIN: usize = 8;

pub struct GameEngine<CardStockT: ICardStock> {
    tableau: Rc<RefCell<GameTableau>>,
    stock: CardStockT,
    complete_sequences: Vec<[Card; COMPLETE_SEQUENCE_LENGTH]>,
}

impl<CardStockT: ICardStock> GameEngine<CardStockT> {
    pub fn new(mut stock: CardStockT) -> Self {
        let tableau = Rc::new(RefCell::new(GameTableau::new(stock.take_initial_cards())));

        Self {
            stock,
            tableau,
            complete_sequences: vec![],
        }
    }
    pub fn tableau(&self) -> Rc<RefCell<GameTableau>> {
        Rc::clone(&self.tableau)
    }

    pub fn deals_left(&self) -> usize {
        self.stock.deals_left()
    }

    pub fn deal_cards(&mut self) {
        if let Some(cards) = self.stock.take_deal() {
            self.tableau.borrow_mut().take_deal(cards);
        } else {
            panic!("No deals left :(");
        }
    }

    pub fn get_available_moves(&self) -> Vec<CardMove> {
        self.tableau.borrow().calculate_available_moves()
    }

    pub fn perform_move(&mut self, card_move: CardMove) -> Result<(), ()> {
        self.tableau.borrow_mut().perform_move(card_move)
    }

    pub fn search_and_update_complete_sequences(&mut self) {
        self.complete_sequences
            .extend(self.tableau.borrow_mut().extract_complete_sequences());
    }

    pub fn is_won(&self) -> bool {
        self.complete_sequences.len() >= COMPLETE_SEQUENCES_TO_WIN
    }
}

/// ------ Formatting ------ ///
impl<CardStockT: ICardStock + std::fmt::Display> fmt::Display for GameEngine<CardStockT> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<GameEngine>")?;
        writeln!(f, "{}", self.stock)?;
        writeln!(f, "{}", self.tableau.borrow())?;
        writeln!(f, "Complete sequences: {}", self.complete_sequences.len())?;
        writeln!(f, "Deals left: {}", self.stock.deals_left())?;
        writeln!(f, "Won?: {}", self.is_won())?;
        writeln!(f, "Available moves: (TODO fmt)")?;
        writeln!(f, "</GameEngine>")
    }
}
