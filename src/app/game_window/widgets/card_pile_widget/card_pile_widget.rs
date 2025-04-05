use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, StatefulWidget, Widget},
};

use crate::{app::game_window::game_cursor::GameCursor, game::v2::CardPileV2};

use super::card_formatting::make_card_pile_ascii_cards;

#[derive(Clone, Copy)]
pub struct CardPileWidget {
    cursor: GameCursor,
}

impl CardPileWidget {
    pub fn new(cursor: GameCursor) -> Self {
        Self { cursor }
    }
}

impl StatefulWidget for CardPileWidget {
    // TODO: Change to struct containing Card slice and necessary data only?
    type State = CardPileV2;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let ascii_cards = make_card_pile_ascii_cards(&self.cursor, state);

        Widget::render(List::new(ascii_cards), area, buf)
    }
}
