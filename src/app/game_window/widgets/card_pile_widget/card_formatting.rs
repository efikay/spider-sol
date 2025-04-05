#![allow(dead_code)]

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

use crate::{
    app::game_window::game_cursor::{GameCursor, GameCursorMode},
    game::{
        core::{Card, Suit},
        v2::CardPileV2,
    },
};

pub fn make_card_pile_ascii_cards<'a>(
    cursor: &'a GameCursor,
    pile: &'a mut CardPileV2,
) -> Vec<Text<'a>> {
    let mut ascii_cards = vec![];

    let this_pile_index = pile.index();
    let cards_len = pile.cards().len();

    let (idx_card_highlight_start, highlight_last_card) = match cursor.pile_index() {
        Some(pile_index) if pile_index == this_pile_index => match cursor.mode() {
            Some(GameCursorMode::CardSelect(active_cards_tail_len))
                if cursor.card_index().is_some() =>
            {
                (
                    Some(
                        cards_len - active_cards_tail_len[this_pile_index]
                            + cursor.card_index().unwrap(),
                    ),
                    false,
                )
            }
            Some(GameCursorMode::PileSelect(piles_filter)) => (None, piles_filter[this_pile_index]),
            _ => (None, false),
        },
        _ => (None, false),
    };

    for (card_index, card) in pile.cards().iter().enumerate() {
        let is_last = card_index == cards_len - 1;
        let is_highlighted = (highlight_last_card && is_last)
            || match idx_card_highlight_start {
                Some(highlight_start_idx) => card_index >= highlight_start_idx,
                None => false,
            };

        ascii_cards.push(make_ascii_card(MakeAsciiCardParams {
            card,
            is_highlighted,
            is_last,
        }));
    }

    ascii_cards
}

struct MakeAsciiCardParams<'a> {
    pub card: &'a Card,
    pub is_highlighted: bool,
    pub is_last: bool,
}
fn make_ascii_card(params: MakeAsciiCardParams) -> Text {
    let MakeAsciiCardParams {
        card,
        is_highlighted,
        is_last,
    } = params;

    let style = get_style_by_highlight(is_highlighted);

    if card.is_opened {
        make_ascii_opened_card(MakeAsciiOpenedCardParams {
            style,
            is_last,
            rank: card.rank.to_human(),
            suit: card.suit.symbol().into(),
            suit_style: Style::new().fg(get_suit_color(card.suit)),
        })
    } else {
        make_ascii_closed_card(&MakeAsciiClosedCardParams {
            is_last,
            style,
            card_back_style: Style::default().fg(Color::Green),
        })
    }
}

/// ----- ASCII opened card ----- ///
struct MakeAsciiOpenedCardParams {
    pub style: Style,
    pub is_last: bool,
    pub rank: String,
    pub suit: String,
    pub suit_style: Style,
}
fn make_ascii_opened_card(params: MakeAsciiOpenedCardParams) -> Text<'static> {
    let MakeAsciiOpenedCardParams {
        style,
        is_last,
        rank,
        suit,
        suit_style,
    } = params;

    let top_line = Line::from("┌─────┐").style(style);
    let wall = Span::styled("│", style);
    let bottom_line = Line::from("└─────┘").style(style);

    if is_last {
        Text::from(vec![
            top_line,
            Line::from(vec![
                wall.clone(),
                Span::styled(format!("{:<5}", rank), suit_style),
                wall.clone(),
            ]),
            Line::from(vec![
                wall.clone(),
                Span::from("  "),
                Span::styled(suit, suit_style),
                Span::from("  "),
                wall.clone(),
            ]),
            Line::from(vec![
                wall.clone(),
                Span::styled(format!("{:>5}", rank), suit_style),
                wall,
            ]),
            bottom_line,
        ])
    } else {
        Text::from(vec![
            top_line,
            Line::from(vec![
                wall.clone(),
                Span::styled(format!("{:<3}{}", rank, suit), suit_style),
                Span::from(" "),
                wall,
            ]),
        ])
    }
}

/// ----- ASCII closed card ----- ///
struct MakeAsciiClosedCardParams {
    pub style: Style,
    pub card_back_style: Style,
    pub is_last: bool,
}
fn make_ascii_closed_card(params: &MakeAsciiClosedCardParams) -> Text<'static> {
    let MakeAsciiClosedCardParams {
        style,
        card_back_style,
        is_last,
    } = *params;

    let top_line = Line::from("┌─────┐").style(style);
    let card_back_line = Line::from(vec![
        Span::styled("│", style),
        Span::styled("░░░░░", card_back_style),
        Span::styled("│", style),
    ]);

    if is_last {
        let bottom_line = Line::from("└─────┘").style(style);

        Text::from(vec![
            top_line,
            card_back_line.clone(),
            card_back_line.clone(),
            card_back_line,
            bottom_line,
        ])
        .style(style)
    } else {
        Text::from(vec![top_line, card_back_line]).style(style)
    }
}


/// ---------- Styling & colors ---------- ///
fn get_style_by_highlight(is_highlighted: bool) -> Style {
    if is_highlighted {
        Style::new().add_modifier(Modifier::BOLD)
    } else {
        Style::new()
    }
}
fn get_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => Color::LightRed,
        Suit::Spades | Suit::Clubs => Color::LightBlue,
    }
}