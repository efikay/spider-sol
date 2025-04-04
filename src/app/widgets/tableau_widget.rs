#[allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::StatefulWidget,
};

use crate::game::{core::PILES_AMOUNT, game_tableau::GameTableau};

use super::card_pile_widget::CardPileWidget;

#[derive(Clone, Copy, Default)]
pub struct TableauWidget;

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

            CardPileWidget::default().render(*pile_area, buf, pile);
        }
    }
}
