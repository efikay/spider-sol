#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct PlayableCardLocation {
    pub pile_index: usize,
    pub card_index: usize,
}
type PileIndex = usize;

#[derive(Debug)]
pub struct AvailableMove {
    from: PlayableCardLocation,
    to: PileIndex,
}

// Implement equality traits
impl PartialEq for AvailableMove {
    fn eq(&self, other: &Self) -> bool {
        self.from.pile_index == other.from.pile_index
            && self.from.card_index == other.from.card_index
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
                match self.from.card_index.cmp(&other.from.card_index) {
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
            self.from.pile_index, self.from.card_index, self.to
        )
    }
}

impl AvailableMove {
    pub fn new((src_pile_index, src_card_index): (usize, usize), to: usize) -> Self {
        Self {
            from: PlayableCardLocation {
                pile_index: src_pile_index,
                card_index: src_card_index,
            },
            to,
        }
    }

    pub fn from(&self) -> PlayableCardLocation {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }
}
