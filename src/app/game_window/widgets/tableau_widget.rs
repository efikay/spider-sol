#[allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::StatefulWidget,
};

use crate::{
    app::game_window::game_cursor::GameCursor,
    game::{core::PILES_AMOUNT, game_tableau::GameTableau},
};

use super::card_pile_widget::CardPileWidget;

#[derive(Clone, Copy)]
pub struct TableauWidget {
    cursor: GameCursor,
}

impl TableauWidget {
    pub fn new(cursor: GameCursor) -> Self {
        Self { cursor }
    }
}

impl StatefulWidget for TableauWidget {
    type State = Rc<RefCell<GameTableau>>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let piles = &state.borrow().piles();
        let pile_w = (100 / PILES_AMOUNT) as u16;

        for (pile_index, pile_area) in
            Layout::horizontal([Constraint::Percentage(pile_w)].repeat(PILES_AMOUNT))
                .split(area)
                .iter()
                .enumerate()
        {
            let pile = &mut piles.borrow_mut()[pile_index];

            CardPileWidget::new(self.cursor).render(*pile_area, buf, pile);
        }
    }
}
