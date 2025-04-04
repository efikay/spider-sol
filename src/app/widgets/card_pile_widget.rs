
use ratatui::{
    layout::Alignment,
    style::{Modifier, Style},
    widgets::{Block, List, StatefulWidget, Widget, block::Title},
};

use crate::game::v2::CardPileV2;

#[derive(Clone, Copy, Default)]
pub struct CardPileWidget;

impl StatefulWidget for CardPileWidget {
    type State = CardPileV2;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        // TODO: List widget is not enough. We need more flexible highlighting
        Widget::render(
            List::new(state.card_strings())
                .block(Block::bordered().title(
                    Title::from(format!("Pile {}", state.index() + 1)).alignment(Alignment::Center),
                ))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("→"),
            area,
            buf,
        )
    }
}
