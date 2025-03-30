#![allow(dead_code)]

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
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
    pub fn from_value(value: u8) -> Option<Self> {
        Rank::iter().nth((value - 1) as usize)
    }
    pub fn next(self) -> Option<Self> {
        Rank::from_value(self.value() + 1)
    }

    pub fn to_human(self) -> String {
        match self {
            Rank::Ace => "A".to_owned(),
            Rank::Jack => "J".to_owned(),
            Rank::Queen => "Q".to_owned(),
            Rank::King => "K".to_owned(),
            number_rank => number_rank.value().to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameMode {
    OneSuit,
    TwoSuits,
    FourSuits,
}
impl GameMode {
    pub fn to_str(&self) -> &'static str {
        match self {
            GameMode::OneSuit => "1 Suit",
            GameMode::TwoSuits => "2 Suits",
            GameMode::FourSuits => "4 Suits",
        }
    }
}
