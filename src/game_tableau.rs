#![allow(dead_code)]

use crate::card_pile::CardPile;

struct GameTableau {
  piles: [CardPile; 10],
}

impl GameTableau {
  pub fn new() -> Self {
    Self {
      piles: std::array::from_fn(|_| CardPile::new()),
    }
  }
}