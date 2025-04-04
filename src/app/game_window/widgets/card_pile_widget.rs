use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{List, StatefulWidget, Widget},
};

use crate::game::v2::CardPileV2;

#[derive(Clone, Copy, Default)]
pub struct CardPileWidget;

impl StatefulWidget for CardPileWidget {
    type State = CardPileV2;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        // TODO: List widget is not enough. We need more flexible highlighting
        Widget::render(
            List::new(state.ascii_card_strings())
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("→"),
            area,
            buf,
        )
    }
}
