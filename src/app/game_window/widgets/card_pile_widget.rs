use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{List, StatefulWidget, Widget},
};

use crate::{app::game_window::game_cursor::GameCursor, game::v2::CardPileV2};

#[derive(Clone, Copy)]
pub struct CardPileWidget {
    cursor: GameCursor,
}

impl CardPileWidget {
    pub fn new(cursor: GameCursor) -> Self {
        Self { cursor }
    }

    fn make_ascii_items(&self, pile: &mut CardPileV2) -> Vec<String> {
        todo!("Make more flexible highlighting with cursor+pile data");
    }
}

impl StatefulWidget for CardPileWidget {
    type State = CardPileV2;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let items = self.make_ascii_items(state);

        Widget::render(
            List::new(items)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("→"),
            area,
            buf,
        )
    }
}
