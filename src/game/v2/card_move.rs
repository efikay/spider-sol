#![allow(dead_code)]

use core::fmt;
use std::cmp::Ordering;

type SrcCardIndex = usize;
type SrcPileIndex = usize;
type DestPileIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardMoveType {
    OnEmptyPile = 1,
    OnCardPile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardMove {
    move_type: CardMoveType,

    src: SrcPileIndex,
    src_card_index: SrcCardIndex,
    dest: SrcPileIndex,
}

impl CardMove {
    pub fn new_card_move(
        (src, src_card_index): (SrcPileIndex, SrcCardIndex),
        dest: SrcPileIndex,
    ) -> Self {
        Self {
            src,
            dest,
            src_card_index,
            move_type: CardMoveType::OnCardPile,
        }
    }
    pub fn new_pile_move(
        (src, src_card_index): (SrcPileIndex, SrcCardIndex),
        dest: DestPileIndex,
    ) -> Self {
        Self {
            src,
            dest,
            src_card_index,
            move_type: CardMoveType::OnEmptyPile,
        }
    }

    pub fn src_pile(&self) -> SrcPileIndex {
        self.src
    }
    pub fn dest_pile(&self) -> DestPileIndex {
        self.dest
    }
    pub fn src_card(&self) -> SrcCardIndex {
        self.src_card_index
    }
    pub fn move_type(&self) -> CardMoveType {
        self.move_type
    }
    pub fn is_on_card_pile_move(&self) -> bool {
        self.move_type == CardMoveType::OnCardPile
    }
}

/// -------- Formatting -------- ///
impl fmt::Display for CardMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {{ {}->{}, card_idx={} }}",
            self.move_type, self.src, self.dest, self.src_card_index
        )
    }
}
impl fmt::Display for CardMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardMoveType::OnEmptyPile => "OnEmptyPile",
                CardMoveType::OnCardPile => "OnCardPile",
            }
        )
    }
}

/// -------- Ordering -------- ///
impl PartialOrd for CardMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardMove {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.src.cmp(&other.src) {
            Ordering::Equal => match self.dest.cmp(&other.dest) {
                Ordering::Equal => self.move_type.cmp(&other.move_type),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

/// -------- Builder -------- ///
pub struct CardMoveBuilder {
    move_type: Option<CardMoveType>,
    src_pile_index: Option<SrcPileIndex>,
    src_card_index: Option<SrcPileIndex>,
    dest_pile_index: Option<SrcPileIndex>,
}

impl CardMoveBuilder {
    pub fn from_pile(src_pile_index: SrcPileIndex) -> Self {
        Self {
            src_pile_index: Some(src_pile_index),
            move_type: None,
            src_card_index: None,
            dest_pile_index: None,
        }
    }

    pub fn to_card_pile(mut self, pile_index: DestPileIndex) -> Self {
        self.dest_pile_index = Some(pile_index);
        self.move_type = Some(CardMoveType::OnCardPile);

        self
    }

    pub fn using_card(mut self, src_card_index: SrcCardIndex) -> Self {
        self.src_card_index = Some(src_card_index);
        self
    }

    pub fn to_empty_pile(mut self, dest_pile_index: DestPileIndex) -> Self {
        self.dest_pile_index = Some(dest_pile_index);
        self.move_type = Some(CardMoveType::OnEmptyPile);

        self
    }

    pub fn build(&mut self) -> CardMove {
        if self.move_type.is_none() {
            panic!("No move type specified");
        }
        if self.src_card_index.is_none() {
            panic!("No src card specified");
        }
        if self.dest_pile_index.is_none() {
            panic!("No dest pile specified");
        }

        match self.move_type.unwrap() {
            CardMoveType::OnEmptyPile => CardMove::new_pile_move(
                (self.src_pile_index.unwrap(), self.src_card_index.unwrap()),
                self.dest_pile_index.unwrap(),
            ),
            CardMoveType::OnCardPile => CardMove::new_card_move(
                (self.src_pile_index.unwrap(), self.src_card_index.unwrap()),
                self.dest_pile_index.unwrap(),
            ),
        }
    }
}
