pub const PILES_AMOUNT: usize = if cfg!(test) {
    4 // Smaller amount for tests
} else {
    10
};

pub const COMPLETE_SEQUENCE_LENGTH: usize = 13;
pub const COMPLETE_SEQUENCES_TO_WIN: usize = 8;
