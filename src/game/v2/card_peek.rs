#![allow(dead_code)]

use crate::game::core::Card;

#[derive(Debug, Clone, Copy)]
pub struct CardPeek {
    pub index: usize,
    pub card: Card,
}
impl CardPeek {
    pub fn with_card_at_index(card: Card, index: usize) -> Self {
        assert!(!card.is_opened);

        Self { card, index }
    }
}
