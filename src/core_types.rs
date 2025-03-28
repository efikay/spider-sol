#![allow(dead_code)]

use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Suit {
    pub fn color(&self) -> &'static str {
        match self {
            Suit::Hearts => "Red",
            Suit::Spades => "Black",
            Suit::Clubs => "Blue",
            Suit::Diamonds => "Orange",
        }
    }

    pub fn simple_color(&self) -> &'static str {
        match self {
            Suit::Hearts | Suit::Diamonds => "Red",
            Suit::Spades | Suit::Clubs => "Black",
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            Suit::Hearts => '♥',
            Suit::Spades => '♠',
            Suit::Clubs => '♣',
            Suit::Diamonds => '♦',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl Rank {
    pub fn value(self) -> u8 {
        self as u8
    }
}

pub enum GameMode {
    OneSuit,
    TwoSuits,
    FourSuits,
}
