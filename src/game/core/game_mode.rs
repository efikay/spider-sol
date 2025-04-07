use core::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GameMode {
    OneSuit,
    TwoSuits,
    FourSuits,
}
impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_str = match self {
            GameMode::OneSuit => "One suit",
            GameMode::TwoSuits => "Two suits",
            GameMode::FourSuits => "Four suits",
        };

        write!(f, "{}", as_str)
    }
}
