#![allow(dead_code)]

use crate::{card_pile::CardPile, card_stock::InitialCards};

const PILES_AMOUNT: usize = 10;

pub struct GameTableau {
    piles: [CardPile; PILES_AMOUNT],
}

impl GameTableau {
    pub fn new(mut initial_cards: InitialCards) -> Self {
        let piles = GameTableau::init_piles(&mut initial_cards);

        Self { piles }
    }

    fn init_piles(cards: &mut InitialCards) -> [CardPile; PILES_AMOUNT] {
        let mut pile_cards: [CardPile; PILES_AMOUNT] = std::array::from_fn(|_| CardPile::new());
        let mut pile_index = 0;

        cards.face_down_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });
        cards.face_up_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });

        pile_cards
    }

    // TODO: Pile move logic
}
