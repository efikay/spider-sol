#![allow(dead_code)]

use crate::game::core::PILES_AMOUNT;

pub enum GameCursorMode {
    /// Cursor used to select a card
    ///
    /// Value represents ranges of cards which you can iterate across for each pile.
    ///
    /// Example: (let's assume that PILES_AMOUNT=4)
    ///
    /// GameCursorMode::CardSelect([2,0,1,3]) means that:
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
    /// - you're welcome to drop a card on third pile
    PileSelect([bool; PILES_AMOUNT]),
}

pub struct GameCursor {
    mode: Option<GameCursorMode>,

    // Cursor position
    pile_index: Option<usize>,
    // Cursor position
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
                    Some(max_index) if max_index == PILES_AMOUNT - 1 => {
                        // TODO: try moving to most-left?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in pile_index..PILES_AMOUNT {
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
                    Some(max_index) if max_index == PILES_AMOUNT - 1 => {
                        // TODO: try moving to most-left?
                    }
                    Some(pile_index) => {
                        for pile_index_candidate in pile_index..PILES_AMOUNT {
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
                            Some(max_card_index)
                                if max_card_index == (pile_lengths[pile_index] - 1) =>
                            {
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

                        self.pile_index = None;
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
