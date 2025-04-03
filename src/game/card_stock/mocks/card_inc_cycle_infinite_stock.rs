use std::fmt;

use crate::game::card_stock::ICardStock;
use crate::game::core::Rank;
use crate::game::core::{Card, PILES_AMOUNT, Suit};

pub struct CardIncCycleInfiniteStock {
    rank: Rank,
}

impl CardIncCycleInfiniteStock {
    pub fn new(starting_rank: Rank) -> Self {
        Self {
            rank: starting_rank,
        }
    }

    fn next_card(&mut self) -> Card {
        self.rank = self.rank.next().unwrap_or(Rank::Ace);

        Card::new_opened(self.rank, Suit::Hearts)
    }
}

impl ICardStock for CardIncCycleInfiniteStock {
    fn deals_left(&self) -> usize {
        9999
    }

    fn take_deal(&mut self) -> Option<Vec<Card>> {
        let cards = (0..PILES_AMOUNT).map(|_| self.next_card()).collect();

        Some(cards)
    }

    fn take_initial_cards(&mut self) -> Vec<Card> {
        (0..PILES_AMOUNT).map(|_| self.next_card()).collect()
    }
}

/// ---- Formatting ---- ///
impl fmt::Display for CardIncCycleInfiniteStock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CardIncCycleInfiniteStock {{ rank: {} }}",
            self.rank.to_human()
        )
    }
}
