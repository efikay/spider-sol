#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct TopCardLocation {
    pub pile_index: usize,
    pub top_card_index: usize,
}
type CardPileIndex = usize;

#[derive(Debug)]
pub struct AvailableMove {
    from: TopCardLocation,
    to: CardPileIndex,
}

// Implement equality traits
impl PartialEq for AvailableMove {
    fn eq(&self, other: &Self) -> bool {
        self.from.pile_index == other.from.pile_index
            && self.from.top_card_index == other.from.top_card_index
            && self.to == other.to
    }
}

impl Eq for AvailableMove {}

impl PartialOrd for AvailableMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AvailableMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.from.pile_index.cmp(&other.from.pile_index) {
            std::cmp::Ordering::Equal => {
                match self.from.top_card_index.cmp(&other.from.top_card_index) {
                    std::cmp::Ordering::Equal => self.to.cmp(&other.to),
                    ordering => ordering,
                }
            }
            ordering => ordering,
        }
    }
}

impl fmt::Display for AvailableMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AvailableMove {{ from: {{ pile: {}, card_idx: {} }}, to_pile: {}  }}",
            self.from.pile_index, self.from.top_card_index, self.to
        )
    }
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
