use std::fmt;

use crate::game::card_stock::InitialCards;
use crate::game::core::{Card, PILES_AMOUNT, Suit};
use crate::game::{card_stock_trait::ICardStock, core::Rank};

pub struct SameCardIncDealStock {
    rank: Rank,
}

impl SameCardIncDealStock {
    pub fn new(starting_rank: Rank) -> Self {
        Self {
            rank: starting_rank,
        }
    }
}

impl fmt::Display for SameCardIncDealStock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SameCardIncDealStock {{ rank: {} }}",
            self.rank.to_human()
        )
    }
}

impl ICardStock for SameCardIncDealStock {
    fn deals_left(&self) -> usize {
        999
    }

    fn take_deal(&mut self) -> Option<Vec<Card>> {
        let same_cards = (0..PILES_AMOUNT)
            .map(|_| Card::new_opened(self.rank, Suit::Spades))
            .collect();
        self.rank = self.rank.next().unwrap_or(Rank::Ace);

        Some(same_cards)
    }

    fn take_initial_cards(&mut self) -> InitialCards {
        InitialCards {
            face_down_cards: vec![],
            face_up_cards: (0..PILES_AMOUNT)
                .map(|_| Card::new_opened(self.rank, Suit::Spades))
                .collect(),
        }
    }
}
