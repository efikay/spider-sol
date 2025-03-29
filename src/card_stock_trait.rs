use crate::{card_stock::InitialCards, core::Card};

pub trait ICardStock {
    fn deals_left(&self) -> usize;
    fn take_deal(&mut self) -> Option<Vec<Card>>;
    fn take_initial_cards(&mut self) -> InitialCards;
}
