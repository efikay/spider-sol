use std::fmt;

use crate::game::card_stock::ICardStock;
use crate::game::core::Rank;
use crate::game::core::{Card, PILES_AMOUNT, Suit};

pub struct SameCardDecDealStock {
    rank: Rank,
}

impl SameCardDecDealStock {
    pub fn new() -> Self {
        Self {
            rank: Rank::King,
        }
    }
}

impl ICardStock for SameCardDecDealStock {
    fn deals_left(&self) -> usize {
        999
    }

    fn take_deal(&mut self) -> Option<Vec<Card>> {
        let same_cards = (0..PILES_AMOUNT)
            .map(|_| Card::new_opened(self.rank, Suit::Spades))
            .collect();
        self.rank = self.rank.prev().unwrap_or(Rank::King);

        Some(same_cards)
    }

    fn take_initial_cards(&mut self) -> Vec<Card> {
        (0..PILES_AMOUNT)
            .map(|_| Card::new_opened(self.rank, Suit::Spades))
            .collect()
    }
}

// ------ Formatting ------ ///
impl fmt::Display for SameCardDecDealStock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SameCardIncDealStock {{ rank: {} }}",
            self.rank.to_human()
        )
    }
}
