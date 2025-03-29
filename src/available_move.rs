#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct TopCardLocation {
    pile_index: usize,
    top_card_index: usize,
}
type CardPileIndex = usize;

#[derive(Debug)]
pub struct AvailableMove {
    from: TopCardLocation,
    to: CardPileIndex,
}

impl AvailableMove {
    pub fn new((src_pile_index, src_top_card_index): (usize, usize), to: usize) -> Self {
        Self {
            from: TopCardLocation {
                pile_index: src_pile_index,
                top_card_index: src_top_card_index,
            },
            to,
        }
    }

    pub fn from(&self) -> TopCardLocation {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }
}
