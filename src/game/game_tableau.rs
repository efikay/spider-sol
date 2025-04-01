#![allow(dead_code)]

use core::fmt;

use crate::game::{
    card_pile::CardPile, card_sequence::CardSequence, card_stock::InitialCards, core::Card,
    data_types::AvailableMove,
};

use super::core::PILES_AMOUNT;

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
        let mut moves = self.calculate_card_on_card_moves();
        moves.extend(self.calculate_empty_pile_moves());

        moves
    }

    fn calculate_card_on_card_moves(&self) -> Vec<AvailableMove> {
        let top_pile_cards: Vec<Vec<Card>> =
            self.piles.iter().map(|pile| pile.top_cards()).collect();
        let mut available_moves = vec![];

        for (src_pile_idx, src_cards) in top_pile_cards.iter().enumerate() {
            for dest_pile_idx in 0..PILES_AMOUNT {
                if dest_pile_idx == src_pile_idx {
                    continue;
                }

                if let Some(dest_tip_card) = &top_pile_cards[dest_pile_idx].last() {
                    for (src_card_idx, src_card) in src_cards.iter().rev().enumerate() {
                        if src_card.can_move_on(&dest_tip_card) {
                            available_moves.push(AvailableMove::new(
                                (src_pile_idx, src_card_idx),
                                dest_pile_idx,
                            ));
                            break;
                        }
                    }
                };
            }
        }

        available_moves
    }
    fn calculate_empty_pile_moves(&self) -> Vec<AvailableMove> {
        let top_pile_cards: Vec<Vec<Card>> =
            self.piles.iter().map(|pile| pile.top_cards()).collect();
        let mut available_moves = vec![];

        for (src_pile_idx, src_cards) in top_pile_cards.iter().enumerate() {
            for dest_pile_idx in 0..PILES_AMOUNT {
                let dest_pile = &top_pile_cards[dest_pile_idx];

                if dest_pile_idx == src_pile_idx || !dest_pile.is_empty() {
                    continue;
                }

                for (src_card_idx, _) in src_cards.iter().enumerate() {
                    available_moves.push(AvailableMove::new(
                        (src_pile_idx, src_card_idx),
                        dest_pile_idx,
                    ));
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
                Card::new_opened(Rank::King, Suit::Spades),
            ],
        });

        let mut expected_moves = vec![
            AvailableMove::new((0, 0), 1),
            AvailableMove::new((0, 0), 3),
            AvailableMove::new((1, 0), 3),
            AvailableMove::new((2, 0), 3),
        ];
        expected_moves.sort();

        let mut result = tableau.calculate_available_moves();
        result.sort();

        assert_eq!(expected_moves, result);
    }

    #[test]
    fn test_simple_card_move() {
        let tableau = GameTableau::from_piles([
            CardPile::from_cards(vec![Card::new_opened(Rank::Two, Suit::Hearts)]),
            CardPile::from_cards(vec![Card::new_opened(Rank::Three, Suit::Hearts)]),
            CardPile::from_cards(vec![]),
            CardPile::from_cards(vec![]),
        ]);

        let expected_card_moves = vec![AvailableMove::new((0, 0), 1)];

        let result = tableau.calculate_card_on_card_moves();

        assert_eq!(expected_card_moves, result);
    }

    #[test]
    fn test_harder_card_move() {
        let tableau = GameTableau::from_piles([
            CardPile::from_cards(vec![
                Card::new_opened(Rank::Two, Suit::Hearts),
                Card::new_opened(Rank::Three, Suit::Hearts),
                Card::new_opened(Rank::Four, Suit::Hearts),
                Card::new_opened(Rank::Five, Suit::Hearts),
            ]),
            CardPile::from_cards(vec![
                Card::new_opened(Rank::Four, Suit::Hearts),
                Card::new_opened(Rank::Five, Suit::Hearts),
                Card::new_opened(Rank::Six, Suit::Hearts),
            ]),
            CardPile::from_cards(vec![]),
            CardPile::from_cards(vec![]),
        ]);

        let expected_card_moves = vec![AvailableMove::new((0, 1), 1)];

        let result = tableau.calculate_card_on_card_moves();

        assert_eq!(expected_card_moves, result);
    }
}
