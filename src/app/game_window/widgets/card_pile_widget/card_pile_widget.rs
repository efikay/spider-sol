use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{List, StatefulWidget, Widget},
};

use crate::{app::game_window::game_cursor::GameCursor, game::v2::CardPileV2};

use super::{
    card_formatting::make_card_pile_ascii_cards,
    pile_selection_formatting::{
        MakeCardPileSelectionArrowsParams, make_card_pile_selection_arrows,
    },
};

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
        let ascii_pile_arrow_lines =
            make_card_pile_selection_arrows(MakeCardPileSelectionArrowsParams {
                pile_index: state.index(),
                cursor_current_pile_index: self.cursor.pile_index(),
                cursor_saved_pile_index: if let Some((_, saved_pile_index)) =
                    self.cursor.get_saved_card_position()
                {
                    Some(saved_pile_index)
                } else {
                    None
                },
            });
        let ascii_card_lines = make_card_pile_ascii_cards(&self.cursor, state);

        let ascii_lines = ascii_card_lines
            .into_iter()
            .chain(ascii_pile_arrow_lines.into_iter())
            .collect::<Vec<_>>();

        Widget::render(List::new(ascii_lines), area, buf)
    }
}
