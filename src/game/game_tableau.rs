#![allow(dead_code)]

use core::fmt;

use crate::game::{
    card_pile::CardPile, card_sequence::CardSequence, card_stock::InitialCards, core::Card,
    data_types::AvailableMove,
};

const PILES_AMOUNT: usize = 10;

pub struct GameTableau {
    piles: [CardPile; PILES_AMOUNT],
}

impl fmt::Display for GameTableau {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<GameTableau>")?;
        writeln!(f, "Piles:")?;
        for (i, pile) in self.piles.iter().enumerate() {
            write!(f, "\t Pile {}:", i)?;
            writeln!(f, "{}", pile)?;
        }
        writeln!(f, "</GameTableau>")
    }
}

impl GameTableau {
    pub fn new(mut initial_cards: InitialCards) -> Self {
        let piles = GameTableau::init_piles(&mut initial_cards);

        Self { piles }
    }
    pub fn from_piles(piles: [CardPile; PILES_AMOUNT]) -> Self {
        Self { piles }
    }

    pub fn take_deal(&mut self, cards: Vec<Card>) {
        self.piles
            .iter_mut()
            .zip(cards.iter())
            .for_each(|(pile, card)| {
                pile.add_deal_card(*card);
            });
    }

    pub fn extract_complete_sequences(&mut self) -> Vec<CardSequence> {
        let mut complete_sequences = vec![];

        for pile in &mut self.piles {
            if let Some(complete_seq) = pile.try_extract_complete_sequence() {
                complete_sequences.push(complete_seq);
            }
        }

        complete_sequences
    }

    fn init_piles(cards: &mut InitialCards) -> [CardPile; PILES_AMOUNT] {
        let mut pile_cards: [CardPile; PILES_AMOUNT] = std::array::from_fn(|_| CardPile::new());
        let mut pile_index = 0;

        cards.face_down_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_start_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });
        cards.face_up_cards.drain(..).for_each(|card| {
            pile_cards[pile_index].add_start_card(card);
            pile_index = (pile_index + 1) % PILES_AMOUNT;
        });

        pile_cards
    }

    // TODO: There's definitely place for optimization (place some breaks at least)
    // TODO: Simplify or move to another module and chunk logic parts
    pub fn calculate_available_moves(&self) -> Vec<AvailableMove> {
        let top_pile_cards: Vec<Vec<Card>> =
            self.piles.iter().map(|pile| pile.top_cards()).collect();
        let mut available_moves = vec![];

        for (src_pile_idx, src_cards) in top_pile_cards.iter().enumerate() {
            for dest_pile_idx in 0..PILES_AMOUNT {
                if dest_pile_idx == src_pile_idx {
                    continue;
                }

                let dest_tip = &top_pile_cards[dest_pile_idx].get(0);

                for (src_card_idx, src_card) in src_cards.iter().enumerate() {
                    match dest_tip {
                        None => {
                            available_moves.push(AvailableMove::new(
                                (src_pile_idx, src_card_idx),
                                dest_pile_idx,
                            ));
                        }
                        Some(dest_tip_card) => {
                            if src_card.can_move_on(&dest_tip_card) {
                                available_moves.push(AvailableMove::new(
                                    (src_pile_idx, src_card_idx),
                                    dest_pile_idx,
                                ));
                            }
                        }
                    }
                }
            }
        }

        available_moves
    }

    // TODO: Pile move logic
}

#[cfg(test)]
mod tests {
    use crate::game::core::{Rank, Suit};

    use super::*;

    #[test]
    fn test_available_moves_simple() {
        let tableau = GameTableau::new(InitialCards {
            face_down_cards: vec![],
            face_up_cards: vec![
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Two, Suit::Spades),
                Card::new_opened(Rank::Three, Suit::Spades),
                Card::new_opened(Rank::Four, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
            ],
        });

        let expected_stacking_moves = vec![
            AvailableMove::new((0, 0), 1),
            AvailableMove::new((1, 0), 2),
            AvailableMove::new((2, 0), 3),
        ];
        let expected_empty_pile_moves = (0..5)
            .flat_map(|pile_index| {
                (5..10)
                    .map(|empty_pile_index| AvailableMove::new((pile_index, 0), empty_pile_index))
                    .collect::<Vec<AvailableMove>>()
            })
            .collect::<Vec<AvailableMove>>();

        let mut expected = Vec::from(expected_stacking_moves);
        expected.extend(expected_empty_pile_moves);
        expected.sort();

        let mut result = tableau.calculate_available_moves();
        result.sort();

        assert_eq!(expected, result);
    }
}
