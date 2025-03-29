#![allow(dead_code)]

use core::fmt;

use crate::{card_sequence::CardSequence, core::Card, data_structures::Stack};

pub struct CardPile {
    sequences: Stack<CardSequence>,
}

impl fmt::Display for CardPile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for card in self.cards() {
            write!(f, "\t{}", card)?;
        }
        write!(f, "")
    }
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

    pub fn try_give_complete_sequence(&mut self) -> Option<CardSequence> {
        let last_seq = self.sequences.peek();

        if last_seq.is_some() && last_seq.unwrap().is_complete() {
            self.sequences.pop()
        } else {
            None
        }
    }

    pub fn add_start_card(&mut self, card: Card) {
        self.add_card(card);
    }
    pub fn add_deal_card(&mut self, card: Card) {
        if self.sequences.is_empty() {
            panic!("cannot add deal card on empty pile!");
        }

        self.add_card(card);
    }
    fn add_card(&mut self, card: Card) {
        match self.sequences.pop() {
            Some(mut last_seq) => {
                let is_card_added = last_seq.add_card(card);
                self.sequences.push(last_seq);

                if !is_card_added {
                    self.sequences.push(CardSequence::from_card(card))
                }
            }
            None => self.sequences.push(CardSequence::from_card(card)),
        }
    }

    fn cards(&self) -> Vec<Card> {
        self.sequences
            .iter()
            .flat_map(|seq| seq.cards.clone())
            .collect()
    }

    // TODO: Pile logic + move-out/move-in logic and full_sequence detection
}
