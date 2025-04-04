#![allow(dead_code)]

use crate::game::core::PILES_AMOUNT;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameCursorMode {
    /// Cursor used to select a card
    ///
    /// Value represents ranges of cards which you can iterate across for each pile.
    ///
    /// Example: (let's assume that PILES_AMOUNT=4)
    ///
    /// GameCursorMode::CardSelect([3,0,1,4]) means that:
    /// - [0] you can iterate over 3 last cards of first pile
    /// - [1] you cannot iterate over second pile
    /// - [2] you can select just the tip of third pile
    /// - [3] you can move across 4 last cards of last pile
    CardSelect([usize; PILES_AMOUNT]),

    /// Cursor used to drop selected card across pile-indexes that are true
    ///
    /// Value represents each pile availability to move in
    ///
    /// Example: (let's assume that PILES_AMOUNT=4)
    ///
    /// GameCursorMode::PlaceCard(vec![true,true,false,true]) means that:
    /// - you can drop selected card on first pile
    /// - you also can drop card on second pile
    /// - you cannot drop a card to third pile
    /// - you're welcome to drop a card on last pile
    PileSelect([bool; PILES_AMOUNT]),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GameCursor {
    mode: Option<GameCursorMode>,

    // Cursor (left-right) position
    pile_index: Option<usize>,
    // Cursor (up-down) position
    card_index: Option<usize>,
}

impl GameCursor {
    pub fn new() -> Self {
        Self {
            mode: None,
            pile_index: None,
            card_index: None,
        }
    }
    pub fn mode(&self) -> &Option<GameCursorMode> {
        &self.mode
    }
    pub fn pile_index(&self) -> Option<usize> {
        self.pile_index
    }
    pub fn card_index(&self) -> Option<usize> {
        self.card_index
    }

    pub fn set_for_card_selection(&mut self, constraints: [usize; PILES_AMOUNT]) {
        self.mode = Some(GameCursorMode::CardSelect(constraints));

        self.recalc_cursor_position();
    }

    pub fn set_for_pile_selection(&mut self, constraints: [bool; PILES_AMOUNT]) {
        self.mode = Some(GameCursorMode::PileSelect(constraints));

        self.recalc_cursor_position();
    }

    // -------- Moves -------- //
    pub fn move_left(&mut self) {
        match self.mode {
            Some(GameCursorMode::CardSelect(pile_lengths)) => {
                match self.pile_index {
                    None => {
                        // Assuming that there were no valid piles to place cursor on. So nowhere to move_left in that case
                        assert!(pile_lengths.iter().all(|len| *len == 0))
                    }
                    Some(0) => {
                        // TODO: try moving to most-right?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in (0..pile_index).rev() {
                            let is_valid = pile_lengths[pile_index_candidate] > 0;
                            if is_valid {
                                if let Some(prev_card_index) = self.card_index {
                                    let is_still_valid =
                                        pile_lengths.get(prev_card_index).is_some();
                                    if is_still_valid {
                                        // Just update the pile_index. card_index is still good
                                        self.pile_index = Some(pile_index_candidate);
                                        return;
                                    }
                                }

                                self.pile_index = Some(pile_index_candidate);
                                self.card_index = Some(0);
                                return;
                            }
                        }
                    }
                }
            }
            Some(GameCursorMode::PileSelect(pile_filters)) => {
                match self.pile_index {
                    None => {
                        assert!(pile_filters.iter().all(|filter| !filter))
                    }
                    Some(0) => {
                        // TODO: try moving to most-right?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in (0..pile_index).rev() {
                            let is_valid = pile_filters[pile_index_candidate];
                            if is_valid {
                                self.pile_index = Some(pile_index_candidate);
                                return;
                            }
                        }
                    }
                }
            }
            None => {
                // No cursor mode selected? Nowhere to move in that case
            }
        }
    }

    pub fn move_right(&mut self) {
        match self.mode {
            Some(GameCursorMode::CardSelect(pile_lengths)) => {
                match self.pile_index {
                    None => {
                        // Assuming that there were no valid piles to place cursor on. So nowhere to move_right in that case
                        assert!(pile_lengths.iter().all(|len| *len == 0))
                    }
                    Some(max_index) if max_index == (PILES_AMOUNT - 1) => {
                        // TODO: try moving to most-left?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in (pile_index + 1)..PILES_AMOUNT {
                            let is_valid = pile_lengths[pile_index_candidate] > 0;
                            if is_valid {
                                if let Some(prev_card_index) = self.card_index {
                                    let is_still_valid =
                                        pile_lengths.get(prev_card_index).is_some();
                                    if is_still_valid {
                                        // Just update the pile_index. card_index is still good
                                        self.pile_index = Some(pile_index_candidate);
                                        return;
                                    }
                                }

                                self.pile_index = Some(pile_index_candidate);
                                self.card_index = Some(0);
                                return;
                            }
                        }
                    }
                }
            }
            Some(GameCursorMode::PileSelect(pile_filters)) => {
                match self.pile_index {
                    None => {
                        assert!(pile_filters.iter().all(|filter| !filter))
                    }
                    Some(max_index) if max_index == (PILES_AMOUNT - 1) => {
                        // TODO: try moving to most-left?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in (pile_index + 1)..PILES_AMOUNT {
                            let is_valid = pile_filters[pile_index_candidate];
                            if is_valid {
                                self.pile_index = Some(pile_index_candidate);
                                return;
                            }
                        }
                    }
                }
            }
            None => {
                // No cursor mode selected? Nowhere to move in that case
            }
        }
    }

    pub fn move_up(&mut self) {
        match self.mode {
            Some(GameCursorMode::CardSelect(pile_lengths)) => {
                match self.card_index {
                    Some(0) => {
                        // No-no-no...
                        // TODO: Try to "teleport" to down-most card?
                    }
                    Some(index) => {
                        self.card_index = Some(index - 1);
                    }
                    None => {
                        // No index? That's only because there were no any non-empty pile to move on
                        assert!(pile_lengths.iter().all(|len| *len == 0))
                    }
                }
            }
            Some(GameCursorMode::PileSelect(_)) => {
                // No reason to move_up when choosing pile
            }
            None => {
                // Cannot move_up just because
            }
        }
    }

    pub fn move_down(&mut self) {
        match self.mode {
            Some(GameCursorMode::CardSelect(pile_lengths)) => {
                match self.pile_index {
                    Some(pile_index) => {
                        match self.card_index {
                            Some(max_card_index) if max_card_index == pile_lengths[pile_index] => {
                                // No-no-no...
                                // TODO: Try to "teleport" to up-most card?
                            }
                            Some(index) => {
                                self.card_index = Some(index + 1);
                            }
                            None => {
                                // No index? That's only because there were no any non-empty pile to move on
                                assert!(pile_lengths.iter().all(|len| *len == 0))
                            }
                        }
                    }
                    None => {
                        // No index? That's only because there were no any non-empty pile to move on
                        assert!(pile_lengths.iter().all(|len| *len == 0))
                    }
                }
            }
            Some(GameCursorMode::PileSelect(_)) => {
                // No reason to move_down when choosing pile
            }
            None => {
                // Cannot move_down just because
            }
        }
    }

    // -------- Cursor recalc -------- //
    fn recalc_cursor_position(&mut self) {
        // Order matters

        self.recalc_pile_index();
        self.recalc_card_index();
    }

    fn recalc_pile_index(&mut self) {
        let pile_filters = match &self.mode {
            Some(GameCursorMode::CardSelect(pile_sizes)) => &pile_sizes.map(|len| len > 0),
            Some(GameCursorMode::PileSelect(pile_validities)) => pile_validities,
            None => {
                return;
            }
        };

        if let Some(prev_pile_index) = self.pile_index {
            let is_still_valid = pile_filters[prev_pile_index];
            if is_still_valid {
                return;
            }
            // else find closest valid?
        }

        for (pile_index, is_valid) in pile_filters.iter().enumerate() {
            if *is_valid {
                self.pile_index = Some(pile_index);
                return;
            }
        }

        self.pile_index = None;
        return;
    }

    /**
     * Assuming pile_index is fresh (should be called right after `recalc_pile_index`)
     */
    fn recalc_card_index(&mut self) {
        match self.mode {
            Some(GameCursorMode::CardSelect(pile_lengths)) => {
                match self.pile_index {
                    Some(pile_index) => {
                        assert!(pile_lengths[pile_index] > 0);

                        if let Some(card_index) = self.card_index {
                            let is_still_valid = card_index <= pile_lengths[pile_index];
                            if is_still_valid {
                                return;
                            }
                        }

                        self.card_index = Some(0);
                        return;
                    }
                    None => {
                        // Assuming that there were no valid piles to place cursor on. So no cards in that case
                        self.card_index = None;
                    }
                }
            }
            Some(GameCursorMode::PileSelect(_)) => {
                // Leave as is (It's not need for that mode anyways)
            }
            None => {
                // Leave as is
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_empty_initially() {
        let cursor = GameCursor::new();

        assert!(cursor.mode.is_none());
        assert!(cursor.pile_index.is_none());
        assert!(cursor.card_index.is_none());
    }

    #[test]
    fn should_set_up_correctly() {
        {
            let pile_lengths: [usize; PILES_AMOUNT] = [1, 1, 1, 1];
            let mut cursor = GameCursor::new();

            cursor.set_for_card_selection(pile_lengths);
            assert_eq!(cursor.mode, Some(GameCursorMode::CardSelect(pile_lengths)));
            assert_eq!(cursor.pile_index, Some(0));
            assert_eq!(cursor.card_index, Some(0));
        }

        let pile_filters = [true, true, true, true];
        let mut cursor = GameCursor::new();

        cursor.set_for_pile_selection(pile_filters);
        assert_eq!(cursor.mode, Some(GameCursorMode::PileSelect(pile_filters)));
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, None); // Should stay untouched
    }

    #[test]
    fn should_move_right_left_correctly() {
        let pile_lengths: [usize; PILES_AMOUNT] = [1, 1, 1, 1];
        let pile_filters = pile_lengths.map(|p| p > 0);

        let mut cursor = GameCursor::new();

        cursor.set_for_card_selection(pile_lengths);

        // Let's try to go left once
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0));

        // Go spam right button
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(1));
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(2));
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));

        // And now we fly to the left
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(2));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(1));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));

        // And now we're trying pile mode
        cursor.set_for_pile_selection(pile_filters);
        assert_eq!(cursor.mode, Some(GameCursorMode::PileSelect(pile_filters)));
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0)); // Should stay untouched

        // Go spam right button AGAIN
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(1));
        assert_eq!(cursor.card_index, Some(0)); // Still untouched
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(2));
        assert_eq!(cursor.card_index, Some(0)); // And even now
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        assert_eq!(cursor.card_index, Some(0)); // Basically all the time in PileMode
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        assert_eq!(cursor.card_index, Some(0)); // Index shall stay the same
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        assert_eq!(cursor.card_index, Some(0)); // ... You know

        // LAST TIME. WE GO LEFT
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(2));
        assert_eq!(cursor.card_index, Some(0)); // Yes I'm schizo maybe ...
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(1));
        assert_eq!(cursor.card_index, Some(0)); // Did I ever tell you..
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0)); // the definition...
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0)); // OF INSANITY? ...
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0)); // 400+ lines already? ...
    }

    #[test]
    fn should_move_up_down_correctly() {
        let pile_lengths: [usize; PILES_AMOUNT] = [4, 0, 0, 0];
        let pile_filters = pile_lengths.map(|p| p > 0);

        let mut cursor = GameCursor::new();

        cursor.set_for_card_selection(pile_lengths);

        // Let's press UP once
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0));

        // Go down-down the dream
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(1));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(2));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(3));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(4));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(4));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(4));
        assert_eq!(cursor.pile_index, Some(0));

        // Stairwayyy to heaven
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(3));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(2));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(1));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(0));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(0));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_up();
        assert_eq!(cursor.card_index, Some(0));
        assert_eq!(cursor.pile_index, Some(0));

        // Very funny... It cannot move up-down but let's test it out!
        cursor.set_for_pile_selection(pile_filters);

        // Go spam UP button once
        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(0));
        assert_eq!(cursor.card_index, Some(0));

        // Go down let's say... twice
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(0));
        assert_eq!(cursor.pile_index, Some(0));
        cursor.move_down();
        assert_eq!(cursor.card_index, Some(0));
        assert_eq!(cursor.pile_index, Some(0));
    }

    #[test]
    fn should_jump_over_empty_piles() {
        let pile_lengths: [usize; PILES_AMOUNT] = [5, 0, 0, 1];
        let pile_filters = pile_lengths.map(|p| p > 0);

        let mut cursor = GameCursor::new();

        cursor.set_for_card_selection(pile_lengths);

        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));

        cursor.set_for_pile_selection(pile_filters);

        cursor.move_right();
        assert_eq!(cursor.pile_index, Some(3));
        cursor.move_left();
        assert_eq!(cursor.pile_index, Some(0));
    }

    #[test]
    fn should_recalc_pile_correctly_tricky() {
        {
            let pile_lengths: [usize; PILES_AMOUNT] = [0, 0, 1, 0];
            let mut cursor = GameCursor::new();

            cursor.set_for_card_selection(pile_lengths);
            assert_eq!(cursor.mode, Some(GameCursorMode::CardSelect(pile_lengths)));
            assert_eq!(cursor.pile_index, Some(2));
        }

        let pile_lengths: [usize; PILES_AMOUNT] = [0, 0, 0, 0];
        let mut cursor = GameCursor::new();

        cursor.set_for_card_selection(pile_lengths);
        assert_eq!(cursor.mode, Some(GameCursorMode::CardSelect(pile_lengths)));
        assert_eq!(cursor.pile_index, None);
    }

    #[test]
    #[ignore]
    fn should_use_prev_card_index_after_jump_when_possible() {
        todo!("Tricky functionality and should be tested");
    }

    #[test]
    #[ignore]
    fn should_recalc_correctly_after_updating_constraints() {
        todo!("Tricky functionality and should be tested");
    }

    #[test]
    #[ignore]
    fn should_go_square_correctly() {
        todo!("Tricky functionality and should be tested");
    }

    #[test]
    #[ignore]
    fn should_go_diagonal_correctly() {
        todo!("Tricky functionality and should be tested");
    }
}
