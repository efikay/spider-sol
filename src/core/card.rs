#![allow(dead_code)]

use core::fmt;

use strum::IntoEnumIterator;

use super::{Rank, Suit};

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub is_opened: bool, // false means "card back is shown, card cannot be played"
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}{}>", self.rank.to_human(), self.suit.symbol())
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            is_opened: false,
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

    // Stacking within same sequence. Different suits stacks as card sequences
    pub fn can_stack_on(&self, other: &Card) -> bool {
        self.rank.value() == other.rank.value() - 1 && self.suit == other.suit
    }

    pub fn make_full_sequence_of(suit: Suit) -> Vec<Card> {
        Rank::iter().map(|r| Card::new(r, suit)).collect()
    }
}
