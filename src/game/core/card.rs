#![allow(dead_code)]

use core::fmt;

use strum::IntoEnumIterator;

use crate::utils::str::pad_left;

use super::{Rank, Suit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub is_opened: bool, // false means "card back is shown, card cannot be played"
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            is_opened: false,
        }
    }
    pub fn new_opened(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            is_opened: true,
        }
    }

    pub fn simple_color(&self) -> &'static str {
        match self.suit {
            Suit::Hearts | Suit::Diamonds => "Red",
            Suit::Spades | Suit::Clubs => "Black",
        }
    }

    pub fn color(&self) -> &'static str {
        match self.suit {
            Suit::Hearts => "Red",
            Suit::Spades => "Black",
            Suit::Clubs => "Orange",
            Suit::Diamonds => "Blue",
        }
    }

    // Can card be stacked within same sequence card
    pub fn can_stack_on(&self, other: &Card) -> bool {
        self.rank.value() == other.rank.value() - 1 && self.suit == other.suit
    }
    // Can card be stacked with any other card
    pub fn can_move_on(&self, other: &Card) -> bool {
        other.is_opened && self.rank.value() + 1 == other.rank.value()
    }

    pub fn make_complete_sequence_of(suit: Suit) -> Vec<Card> {
        Rank::iter().map(|r| Card::new(r, suit)).collect()
    }
    pub fn make_complete_sequence_of_opened(suit: Suit) -> Vec<Card> {
        Rank::iter().map(|r| Card::new_opened(r, suit)).collect()
    }
}

// ------ Formatting ------ ///
impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.is_opened {
            return write!(f, "###");
        }

        write!(
            f,
            "{}{}",
            pad_left(self.rank.to_human().as_str(), 2, ' '),
            self.suit.symbol(),
        )
    }
}
