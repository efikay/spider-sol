pub const PILES_AMOUNT: usize = if cfg!(test) {
    4 // Smaller amount for tests
} else {
    10
};

pub const FULL_SEQUENCE_LENGTH: usize = 13;
