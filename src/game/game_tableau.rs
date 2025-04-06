#![allow(dead_code)]

use core::fmt;
use std::{cell::RefCell, rc::Rc};

use super::{
    core::{COMPLETE_SEQUENCE_LENGTH, PILES_AMOUNT},
    v2::{CardMove, CardMoveType, CardPileV2},
};
use crate::game::core::Card;

pub struct GameTableau {
    piles: Rc<RefCell<[CardPileV2; PILES_AMOUNT]>>,
}

impl GameTableau {
    pub fn new(initial_cards: Vec<Card>) -> Self {
        Self {
            piles: GameTableau::init_piles(initial_cards),
        }
    }
    pub fn from_piles(piles: [CardPileV2; PILES_AMOUNT]) -> Self {
        let piles = Rc::new(RefCell::new(piles));

        Self { piles }
    }
    pub fn from_empty_piles() -> Self {
        Self {
            piles: Rc::new(RefCell::new(CardPileV2::init_piles())),
        }
    }

    pub fn piles(&self) -> Rc<RefCell<[CardPileV2; PILES_AMOUNT]>> {
        Rc::clone(&self.piles)
    }

    pub fn take_deal(&mut self, cards: Vec<Card>) {
        self.piles
            .borrow_mut()
            .iter_mut()
            .zip(cards.iter())
            .for_each(|(pile, card)| {
                pile.add_deal_card(*card);
            });
    }

    pub fn extract_complete_sequences(&mut self) -> Vec<[Card; COMPLETE_SEQUENCE_LENGTH]> {
        let mut complete_sequences = vec![];

        for pile in self.piles.borrow_mut().iter_mut() {
            if let Some(complete_seq) = pile.try_extract_complete_sequence() {
                complete_sequences.push(complete_seq);
            }
        }

        complete_sequences
    }

    fn init_piles(cards: Vec<Card>) -> Rc<RefCell<[CardPileV2; PILES_AMOUNT]>> {
        let mut pile_cards = CardPileV2::init_piles();

        cards.into_iter().enumerate().for_each(|(index, card)| {
            pile_cards[index % PILES_AMOUNT].add_start_card(card);
        });

        Rc::new(RefCell::new(pile_cards))
    }

    pub fn calculate_available_moves(&self) -> Vec<CardMove> {
        let mut available_moves = vec![];

        for pile in self.piles.borrow().iter() {
            for other_pile in self.piles.borrow().iter() {
                if other_pile == pile {
                    continue;
                }

                available_moves.extend(pile.calc_moves_to(other_pile));
            }
        }

        available_moves
    }

    pub fn perform_move(&mut self, card_move: CardMove) -> Result<(), ()> {
        let src_pile: &mut CardPileV2 =
            unsafe { &mut *(&mut self.piles.borrow_mut()[card_move.src_pile()] as *mut _) };
        let dest_pile: &mut CardPileV2 =
            unsafe { &mut *(&mut self.piles.borrow_mut()[card_move.dest_pile()] as *mut _) };

        match card_move.move_type() {
            CardMoveType::OnEmptyPile(src_card) => {
                src_pile.perform_empty_pile_move(dest_pile, src_card)
            }
            CardMoveType::OnCardPile => src_pile.perform_card_pile_move(dest_pile),
        }
    }
}

/// ------ Formatting ------ ///
impl fmt::Display for GameTableau {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "<GameTableau>")?;
        writeln!(f, "Piles:")?;
        for (i, pile) in self.piles.borrow().iter().enumerate() {
            write!(f, "\t Pile {}:", i)?;
            writeln!(f, "{}", pile)?;
        }
        writeln!(f, "</GameTableau>")
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{
        core::{Rank, Suit},
        v2::CardMoveBuilder,
    };

    use super::*;

    #[test]
    fn should_calculate_moves_simple_case() {
        let tableau = GameTableau::new(vec![
            Card::new_opened(Rank::Ace, Suit::Spades),
            Card::new_opened(Rank::Two, Suit::Spades),
            Card::new_opened(Rank::King, Suit::Spades),
        ]);

        let mut expected_moves = vec![
            CardMoveBuilder::from_pile(0).to_card_pile(1).build(),
            CardMoveBuilder::from_pile(0)
                .using_card(0)
                .to_empty_pile(3)
                .build(),
            CardMoveBuilder::from_pile(1)
                .using_card(0)
                .to_empty_pile(3)
                .build(),
            CardMoveBuilder::from_pile(2)
                .using_card(0)
                .to_empty_pile(3)
                .build(),
        ];
        expected_moves.sort();

        let mut result = tableau.calculate_available_moves();
        result.sort();

        assert_eq!(expected_moves, result);
    }

    #[test]
    fn should_calculate_single_card_pile_move() {
        let tableau = GameTableau::new(vec![
            Card::new_opened(Rank::Two, Suit::Hearts),
            Card::new_opened(Rank::Three, Suit::Hearts),
        ]);

        let expected_card_moves = vec![CardMoveBuilder::from_pile(0).to_card_pile(1).build()];

        let result_card_moves: Vec<CardMove> = tableau
            .calculate_available_moves()
            .iter()
            .filter(|m| m.is_on_card_pile_move())
            .cloned()
            .collect();

        assert_eq!(expected_card_moves, result_card_moves);
    }

    #[test]
    fn should_calculate_tricky_card_pile_moves() {
        let tableau = GameTableau::from_piles([
            CardPileV2::from_cards(
                vec![
                    Card::new_opened(Rank::Five, Suit::Hearts),
                    Card::new_opened(Rank::Four, Suit::Hearts),
                    Card::new_opened(Rank::Three, Suit::Hearts),
                    Card::new_opened(Rank::Two, Suit::Hearts),
                ],
                0,
            ),
            CardPileV2::from_cards(
                vec![
                    Card::new_opened(Rank::Six, Suit::Hearts),
                    Card::new_opened(Rank::Five, Suit::Hearts),
                    Card::new_opened(Rank::Four, Suit::Hearts),
                ],
                1,
            ),
            CardPileV2::from_cards(vec![], 2),
            CardPileV2::from_cards(vec![], 3),
        ]);

        let expected_card_moves = vec![
            CardMoveBuilder::from_pile(0).to_card_pile(1).build(),
            CardMoveBuilder::from_pile(1).to_card_pile(0).build(),
        ];
        let result_card_moves: Vec<CardMove> = tableau
            .calculate_available_moves()
            .iter()
            .filter(|m| m.is_on_card_pile_move())
            .cloned()
            .collect();

        assert_eq!(expected_card_moves, result_card_moves);
    }
}
