#![allow(dead_code)]

use crate::{
    card_deck::CardDeck, card_sequence::CardSequence, card_stock::CardStock, core::GameMode,
    game_tableau::GameTableau,
};

struct Game {
    tableau: GameTableau,
    stock: CardStock,
    finished_sequences: Vec<CardSequence>,
}

impl Game {
    pub fn new(game_mode: GameMode) -> Self {
        let mut stock = CardStock::new(CardDeck::new(game_mode));
        let tableau = GameTableau::new(stock.take_initial_cards());

        Self {
            stock,
            tableau,
            finished_sequences: vec![],
        }
    }

    // TODO: Game logic
}
