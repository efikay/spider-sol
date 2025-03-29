#![allow(dead_code)]

use core::fmt;

use crate::{
    card_pile::CardPile, card_sequence::CardSequence, card_stock::InitialCards, core::Card,
};

const PILES_AMOUNT: usize = 10;

pub struct GameTableau {
    piles: [CardPile; PILES_AMOUNT],
}

impl fmt::Display for GameTableau {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<GameTableau>")?;
        writeln!(f, "Piles:")?;
        for (i, pile) in self.piles.iter().enumerate() {
            write!(f, "\t Pile {}:", i + 1)?;
            writeln!(f, "{}", pile)?;
        }
        writeln!(f, "</GameTableau>")
    }
}

impl GameTableau {
    pub fn new(mut initial_cards: InitialCards) -> Self {
        let piles = GameTableau::init_piles(&mut initial_cards);

        Self { piles }
    }

    pub fn take_deal(&mut self, cards: Vec<Card>) {
        self.piles
            .iter_mut()
            .zip(cards.iter())
            .for_each(|(pile, card)| {
                pile.add_deal_card(*card);
            });
    }

    pub fn extract_complete_sequences(&mut self) -> Vec<CardSequence> {
        let mut complete_sequences = vec![];

        for pile in &mut self.piles {
            if let Some(complete_seq) = pile.try_extract_complete_sequence() {
                complete_sequences.push(complete_seq);
            }
        }

        complete_sequences
    }

    fn init_piles(cards: &mut InitialCards) -> [CardPile; PILES_AMOUNT] {
        let mut pile_cards: [CardPile; PILES_AMOUNT] = std::array::from_fn(|_| CardPile::new());
        let mut pile_index = 0;

        cards.face_down_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_start_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });
        cards.face_up_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_start_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });

        pile_cards
    }

    // TODO: Pile move logic
}
