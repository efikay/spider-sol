#[allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use ratatui::{
    layout::{Constraint, Layout},
    widgets::StatefulWidget,
};

use crate::game::{core::PILES_AMOUNT, game_tableau::GameTableau};

use super::card_pile_widget::CardPileWidget;

#[derive(Clone, Copy, Default)]
pub struct TableauWidget;

impl StatefulWidget for TableauWidget {
    type State = Rc<RefCell<GameTableau>>;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let piles = &state.borrow().piles();

        for (pile_index, pile_area) in
            Layout::horizontal([Constraint::Percentage(10)].repeat(PILES_AMOUNT))
                .split(area)
                .iter()
                .enumerate()
        {
            let pile = &mut piles.borrow_mut()[pile_index];

            CardPileWidget::default().render(*pile_area, buf, pile);
        }
    }
}
