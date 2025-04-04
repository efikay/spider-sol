use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, PartialOrd, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
impl Suit {
    pub fn symbol(&self) -> char {
        match self {
            Suit::Hearts => '♡',
            Suit::Spades => '♤',
            Suit::Clubs => '♧',
            Suit::Diamonds => '♢',
        }
    }
}
