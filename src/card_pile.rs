#![allow(dead_code)]

use crate::{card_sequence::CardSequence, core::Card, data_structures::Stack};

pub struct CardPile {
    sequences: Stack<CardSequence>,
}

impl CardPile {
    pub fn new() -> Self {
        Self {
            sequences: Stack::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }

    pub fn add_deal_card(&mut self, card: Card) {
        match self.sequences.pop() {
            Some(mut last_seq) => {
                let is_card_added = last_seq.add_card(card);
                self.sequences.push(last_seq);

                if !is_card_added {
                    self.sequences.push(CardSequence::from_card(card))
                }
            }
            None => {
                panic!("Cannot add deal card on empty pile!")
            }
        }
    }

    // TODO: Pile logic + move-out/move-in logic and full_sequence detection
}
