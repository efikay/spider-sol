#![allow(dead_code)]

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

use crate::{
    app::game_window::game_cursor::{GameCursor, GameCursorMode},
    game::{
        core::{Card, Suit},
        v2::{CardPeek, CardPileV2},
    },
};

pub fn make_card_pile_ascii_cards<'a>(
    cursor: &'a GameCursor,
    pile: &'a mut CardPileV2,
    card_peek: Option<CardPeek>,
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
            Some(GameCursorMode::PileSelect(piles_filter)) => {
                (None, piles_filter[this_pile_index] > 0)
            }
            _ => (None, false),
        },
        _ => (None, false),
    };

    for (card_index, card) in pile.cards().iter().enumerate() {
        let is_last = card_index == cards_len - 1;
        let is_picked = if let Some(card_peek) = card_peek {
            card_peek.index == card_index
        } else {
            false
        };

        let border_styling = if is_picked {
            BorderStyling::Picked
        } else {
            match idx_card_highlight_start {
                Some(highlight_start_idx) => match card_index >= highlight_start_idx {
                    true => BorderStyling::Highlight,
                    false => match card.is_opened {
                        true => BorderStyling::Dim,
                        false => BorderStyling::Default,
                    },
                },
                None => match highlight_last_card && is_last {
                    true => BorderStyling::Highlight,
                    false => match cursor.mode() {
                        Some(GameCursorMode::CardSelect(_)) => {
                            assert!(highlight_last_card == false);
                            BorderStyling::Default
                        }
                        Some(GameCursorMode::PileSelect(_)) => match cursor.pile_index() {
                            Some(index) if index == this_pile_index => BorderStyling::Dim,
                            _ => BorderStyling::Default,
                        },
                        None => {
                            assert!(false);
                            BorderStyling::Default
                        }
                    },
                },
            }
        };

        ascii_cards.push(make_ascii_card(MakeAsciiCardParams {
            card,
            border_styling,
            is_last,
        }));
    }

    ascii_cards
}

enum BorderStyling {
    Default,
    Highlight,
    Dim,
    Picked,
}

struct MakeAsciiCardParams<'a> {
    pub card: &'a Card,
    pub border_styling: BorderStyling,
    pub is_last: bool,
}
fn make_ascii_card(params: MakeAsciiCardParams) -> Text {
    let MakeAsciiCardParams {
        card,
        border_styling,
        is_last,
    } = params;

    let border_style = calc_border_style(border_styling);

    if card.is_opened {
        make_ascii_opened_card(MakeAsciiOpenedCardParams {
            border_style,
            is_last,
            rank: card.rank.to_human(),
            suit: card.suit.symbol().into(),
            suit_style: Style::new().fg(get_suit_color(card.suit)),
        })
    } else {
        make_ascii_closed_card(&MakeAsciiClosedCardParams {
            is_last,
            border_style,
            card_back_style: Style::default().fg(Color::Green),
        })
    }
}

/// ----- ASCII opened card ----- ///
struct MakeAsciiOpenedCardParams {
    pub border_style: Style,
    pub is_last: bool,
    pub rank: String,
    pub suit: String,
    pub suit_style: Style,
}
fn make_ascii_opened_card(params: MakeAsciiOpenedCardParams) -> Text<'static> {
    let MakeAsciiOpenedCardParams {
        border_style,
        is_last,
        rank,
        suit,
        suit_style,
    } = params;

    let top_line = Line::from("┌─────┐").style(border_style);
    let wall = Span::styled("│", border_style);
    let bottom_line = Line::from("└─────┘").style(border_style);

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
    pub border_style: Style,
    pub card_back_style: Style,
    pub is_last: bool,
}
fn make_ascii_closed_card(params: &MakeAsciiClosedCardParams) -> Text<'static> {
    let MakeAsciiClosedCardParams {
        border_style,
        card_back_style,
        is_last,
    } = *params;

    let top_line = Line::from("┌─────┐").style(border_style);
    let card_back_line = Line::from(vec![
        Span::styled("│", border_style),
        Span::styled("░░░░░", card_back_style),
        Span::styled("│", border_style),
    ]);

    if is_last {
        let bottom_line = Line::from("└─────┘").style(border_style);

        Text::from(vec![
            top_line,
            card_back_line.clone(),
            card_back_line.clone(),
            card_back_line,
            bottom_line,
        ])
    } else {
        Text::from(vec![top_line, card_back_line])
    }
}

/// ---------- Styling & colors ---------- ///
fn calc_border_style(border_styling: BorderStyling) -> Style {
    match border_styling {
        BorderStyling::Default => Style::new(),
        BorderStyling::Highlight => Style::new().fg(Color::LightMagenta),
        BorderStyling::Dim => Style::new().add_modifier(Modifier::DIM),
        BorderStyling::Picked => Style::new()
            .add_modifier(Modifier::DIM)
            .fg(Color::LightYellow),
    }
}
fn get_suit_color(suit: Suit) -> Color {
    match suit {
        Suit::Hearts | Suit::Diamonds => Color::LightRed,
        Suit::Spades | Suit::Clubs => Color::LightBlue,
    }
}
