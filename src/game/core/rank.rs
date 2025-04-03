use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl Rank {
    pub fn value(self) -> u8 {
        self as u8
    }
    pub fn from_value(value: u8) -> Option<Self> {
        if value > 0 {
            Rank::iter().nth((value - 1) as usize)
        } else {
            None
        }
    }
    pub fn next(&self) -> Option<Self> {
        Rank::from_value(self.value() + 1)
    }
    pub fn prev(&self) -> Option<Self> {
        match self.value().checked_sub(1) {
            Some(prev_value) => Rank::from_value(prev_value),
            None => None,
        }
    }

    pub fn to_human(self) -> String {
        match self {
            Rank::Ace => "A".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
            number_rank => number_rank.value().to_string(),
        }
    }
}
