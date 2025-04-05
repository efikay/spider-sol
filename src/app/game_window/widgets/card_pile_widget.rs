use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{List, StatefulWidget, Widget},
};

use crate::{
    app::game_window::game_cursor::{GameCursor, GameCursorMode},
    game::{core::Card, v2::CardPileV2},
};

#[derive(Clone, Copy)]
pub struct CardPileWidget {
    // TODO: Make specific struct for CardPileWidgetState and move cursor there
    cursor: GameCursor,
}

impl CardPileWidget {
    pub fn new(cursor: GameCursor) -> Self {
        Self { cursor }
    }
}

/// ------ Render ------ ///
impl StatefulWidget for CardPileWidget {
    // TODO: Change to Card slice?
    type State = CardPileV2;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let ascii_cards = make_ascii_cards(&self.cursor, state);

        Widget::render(
            List::new(ascii_cards)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("→"),
            area,
            buf,
        )
    }
}

/// ----- Extremely needs refactoring ----- ///
fn make_ascii_cards<'a>(cursor: &'a GameCursor, pile: &'a mut CardPileV2) -> Vec<Text<'a>> {
    let mut ascii_cards = vec![];
    let pile_index = pile.index();
    let cards_len = pile.cards().len();

    let (active_card_idx, is_pile_highlighted) = match cursor.mode() {
        Some(GameCursorMode::CardSelect(active_cards_tail_len)) => {
            match cursor.card_index() {
                Some(card_idx) => match cursor.pile_index() {
                    Some(active_pile_index) if active_pile_index == pile_index => (
                        Some(cards_len - active_cards_tail_len[pile_index] + card_idx),
                        false,
                    ),
                    _ => (None, false),
                },
                None => (None, false),
            }
        }
        Some(GameCursorMode::PileSelect(piles_filter)) => (None, piles_filter[pile_index]),
        None => (None, false),
    };

    for (card_index, card) in pile.cards().iter().enumerate() {
        let is_last = card_index == cards_len - 1;
        let is_highlighted = is_pile_highlighted
            || match active_card_idx {
                Some(index) => index == card_index,
                None => false,
            };

        ascii_cards.push(make_ascii_card(card, is_highlighted, is_last));
    }

    ascii_cards
}

fn make_ascii_card(card: &Card, is_highlighted: bool, is_last: bool) -> Text {
    let style = if is_highlighted {
        Style::new().add_modifier(Modifier::BOLD)
    } else {
        Style::new()
    };

    if is_last {
        if card.is_opened {
            Text::from(format!(
                "┌─────┐\n\
                 │{:<5}│\n\
                 │  {}  │\n\
                 │{:>5}│\n\
                 └─────┘",
                card.rank.to_human(),
                card.suit.symbol(),
                card.rank.to_human()
            ))
            .style(style)
        } else {
            Text::from(vec![
                Line::from("┌─────┐"),
                Line::from("│░░░░░│"),
                Line::from("│░░░░░│"),
                Line::from("│░░░░░│"),
                Line::from("└─────┘"),
            ])
            .style(style)
        }
    } else {
        if card.is_opened {
            Text::from(vec![
                Line::from("┌─────┐"),
                Line::from(vec![Span::styled(
                    format!("|{:<3}{} |", card.rank.to_human(), card.suit.symbol()),
                    Style::default(),
                )]),
            ])
            .style(style)
        } else {
            Text::from(vec![Line::from("┌─────┐"), Line::from("│░░░░░│")]).style(style)
        }
    }
}
