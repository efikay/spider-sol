#![allow(dead_code)]

use ratatui::{
    style::{Modifier, Style},
    text::Text,
};

pub struct MakeCardPileSelectionArrowsParams {
    pub pile_index: usize,

    pub cursor_saved_pile_index: Option<usize>,
    pub cursor_current_pile_index: Option<usize>,
}
pub fn make_card_pile_selection_arrows(
    params: MakeCardPileSelectionArrowsParams,
) -> Vec<Text<'static>> {
    let MakeCardPileSelectionArrowsParams {
        pile_index,
        cursor_saved_pile_index,
        cursor_current_pile_index,
    } = params;

    let mut ascii_arrows = vec![];

    let is_prev_selected_arrow_needed =
        if let Some(selected_card_pile_index) = cursor_saved_pile_index {
            selected_card_pile_index == pile_index
        } else {
            false
        };
    let is_selected_arrow_needed = if let Some(current_pile_index) = cursor_current_pile_index {
        current_pile_index == pile_index
    } else {
        false
    };

    if is_prev_selected_arrow_needed {
        ascii_arrows
            .push(Text::from("   ↑   ").style(Style::default().add_modifier(Modifier::DIM)));
    }
    if is_selected_arrow_needed {
        ascii_arrows
            .push(Text::from("   ↑   ").style(Style::default().add_modifier(Modifier::BOLD)));
    }

    ascii_arrows
}
