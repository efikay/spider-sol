use crate::core_types::{Rank, Suit};

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub is_opened: bool, // false means "card back is shown, card cannot be played"
}

impl Card {
    pub fn new_opened(rank: Rank, suit: Suit) -> Self {
        Self {
            rank,
            suit,
            is_opened: true,
        }
    }

    pub fn new_closed(rank: Rank, suit: Suit) -> Self {
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

    pub fn can_stack_on(&self, other: Option<&Card>) -> bool {
        match other {
            None => true, // Can be placed on empty pile
            Some(card) => self.rank.value() == card.rank.value() - 1,
        }
    }
}
