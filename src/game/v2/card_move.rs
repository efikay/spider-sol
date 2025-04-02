#![allow(dead_code)]

type SrcCardIndex = usize;
type SrcPileIndex = usize;
type DestPileIndex = usize;

#[derive(Clone, Copy)]
enum CardMoveType {
    OnEmptyPile(SrcCardIndex),
    OnCardPile,
}

#[derive(Clone, Copy)]
pub struct CardMove {
    move_type: CardMoveType,
    src: SrcPileIndex,
    dest: SrcPileIndex,
}

impl CardMove {
    pub fn new_card_move(src: SrcPileIndex, dest: SrcPileIndex) -> Self {
        Self {
            src,
            dest,
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
            move_type: CardMoveType::OnEmptyPile(src_card_index),
        }
    }

    pub fn src_pile(&self) -> SrcPileIndex {
        self.src
    }
    pub fn dest_pile(&self) -> DestPileIndex {
        self.dest
    }
    pub fn src_card(&self) -> Option<SrcCardIndex> {
        match self.move_type {
            CardMoveType::OnEmptyPile(card_index) => Some(card_index),
            CardMoveType::OnCardPile => None,
        }
    }

    pub fn is_pile_move(&self) -> bool {
        match self.move_type {
            CardMoveType::OnEmptyPile(_) => true,
            _ => false,
        }
    }
    pub fn is_card_move(&self) -> bool {
        match self.move_type {
            CardMoveType::OnCardPile => true,
            _ => false,
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
        match self.src_card_index {
            Some(src_card) => {
                self.move_type = Some(CardMoveType::OnEmptyPile(src_card));
            }
            None => panic!("Please specify card before creating empty pile move!"),
        }

        self
    }

    pub fn build(&mut self) -> CardMove {
        if self.move_type.is_none() {
            panic!("No move type specified");
        }
        if self.dest_pile_index.is_none() {
            panic!("No dest pile specified");
        }

        match self.move_type.unwrap() {
            CardMoveType::OnEmptyPile(card_index) => CardMove::new_pile_move(
                (self.src_pile_index.unwrap(), card_index),
                self.dest_pile_index.unwrap(),
            ),
            CardMoveType::OnCardPile => {
                CardMove::new_card_move(self.src_pile_index.unwrap(), self.dest_pile_index.unwrap())
            }
        }
    }
}
