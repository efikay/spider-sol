#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::game::{
    card_stock::ICardStock, core::PILES_AMOUNT, game_engine::GameEngine, v2::CardMoveBuilder,
};

use super::{
    game_cursor::{GameCursor, GameCursorMode},
    widgets::TableauWidget,
};

pub struct GameWindow<CardStockT: ICardStock> {
    game_engine: GameEngine<CardStockT>,

    cursor: GameCursor,
}

impl<CardStockT: ICardStock> GameWindow<CardStockT> {
    pub fn new(stock: CardStockT) -> GameWindow<CardStockT> {
        let game_engine = GameEngine::new(stock);

        let mut cursor = GameCursor::new();
        cursor.set_for_card_selection(std::array::from_fn(|_| 0));

        Self {
            game_engine,
            cursor,
        }
    }

    fn deals_left(&self) -> usize {
        self.game_engine.deals_left()
    }
    fn can_deal_cards(&mut self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::CardSelect(_)) => self.deals_left() > 0,
            _ => false,
        }
    }
    fn deal_cards(&mut self) {
        if self.can_deal_cards() {
            self.game_engine.deal_cards();
        }
    }

    fn is_selecting_a_card(&self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::CardSelect(_)) => true,
            _ => false,
        }
    }
    fn is_placing_a_card(&self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::PileSelect(_)) => true,
            _ => false,
        }
    }

    fn save_current_cursor_position(&mut self) -> Result<(), ()> {
        self.cursor.save_card_position()
    }

    fn attempt_to_place_selected_card_to_current_cursor_position(&mut self) -> Result<(), ()> {
        if let (Some((selected_card_idx, selected_card_pile_idx)), Some(target_pile_idx)) = (
            self.cursor.get_saved_card_position(),
            self.cursor.pile_index(),
        ) {
            let is_target_pile_empty = {
                let tableau = &self.game_engine.tableau();
                let tableau = tableau.borrow();

                let piles = tableau.piles();
                let piles = piles.borrow();

                piles[target_pile_idx].is_empty()
            };

            if is_target_pile_empty {
                self.game_engine.perform_move(
                    CardMoveBuilder::from_pile(selected_card_pile_idx)
                        .using_card(selected_card_idx)
                        .to_empty_pile(target_pile_idx)
                        .build(),
                )
            } else {
                self.game_engine.perform_move(
                    CardMoveBuilder::from_pile(selected_card_pile_idx)
                        .to_card_pile(target_pile_idx)
                        .build(),
                )
            }
        } else {
            Err(())
        }
    }

    fn calc_playable_card_lengths(&self) -> [usize; PILES_AMOUNT] {
        self.game_engine
            .tableau()
            .borrow()
            .piles()
            .borrow()
            .iter()
            .map(|pile| pile.playable_cards_len())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }
    fn update_cursor_constraints(&mut self) {
        match self.cursor.mode() {
            Some(GameCursorMode::CardSelect(_)) => {
                self.cursor
                    .update_constraints(self.calc_playable_card_lengths());
            }
            Some(GameCursorMode::PileSelect(_)) => {
                let all_piles_available = std::array::from_fn(|_| 1);

                self.cursor.update_constraints(all_piles_available);
            }
            _ => {}
        }
    }

    // -- Keys -- //
    pub fn on_key_pressed(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            // [Arrow navigation]
            (_, KeyCode::Left | KeyCode::Char('h')) => self.on_left_pressed(),
            (_, KeyCode::Down | KeyCode::Char('j')) => self.on_down_pressed(),
            (_, KeyCode::Up | KeyCode::Char('k')) => self.on_up_pressed(),
            (_, KeyCode::Right | KeyCode::Char('l')) => self.on_right_pressed(),
            // [Deal cards]
            (_, KeyCode::Char('d')) => self.on_d_pressed(),
            // [Select a card / Select a pile]
            (_, KeyCode::Enter) => self.on_enter_pressed(),
            _ => {}
        }
    }
    fn on_enter_pressed(&mut self) {
        if self.is_selecting_a_card() {
            if let Ok(_) = self.save_current_cursor_position() {
                self.cursor
                    .set_for_pile_selection(self.calc_playable_card_lengths());
            }
        } else if self.is_placing_a_card() {
            if let Ok(_) = self.attempt_to_place_selected_card_to_current_cursor_position() {
                //
            }

            self.cursor
                .set_for_card_selection(self.calc_playable_card_lengths());
        }
    }
    fn on_d_pressed(&mut self) {
        if !self.is_placing_a_card() && self.game_engine.deals_left() > 0 {
            self.deal_cards();
        }
    }
    fn on_up_pressed(&mut self) {
        self.cursor.move_up();
    }
    fn on_down_pressed(&mut self) {
        self.cursor.move_down();
    }
    fn on_left_pressed(&mut self) {
        self.cursor.move_left();
    }
    fn on_right_pressed(&mut self) {
        self.cursor.move_right();
    }

    // --- Render --- //
    pub fn render_window(&mut self, frame: &mut Frame) {
        self.game_engine.search_and_update_complete_sequences();
        self.update_cursor_constraints();

        let areas = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame.area());
        {
            let text_area = areas[0];
            let text = Text::from(vec![
                Line::from(format!(
                    "Deals left: {} {}{}| <q> - exit || Complete sequences - {}",
                    self.game_engine.deals_left(),
                    if self.can_deal_cards() {
                        "| <d> - Take deal "
                    } else {
                        ""
                    },
                    if self.is_selecting_a_card() {
                        "| Select card(-s) "
                    } else if self.is_placing_a_card() {
                        "| Choose a pile to place card(-s) "
                    } else {
                        "| <Err> "
                    },
                    self.game_engine.complete_sequences_count(),
                )),
                Line::from(format!(
                    "<DEBUG>: saved_cursor_position={:?}, cursor_position={:?}",
                    self.cursor.get_saved_card_position(),
                    (self.cursor.card_index(), self.cursor.pile_index())
                )),
            ]);

            let paragraph = Paragraph::new(text)
                .block(Block::bordered())
                .style(Style::default());

            frame.render_widget(paragraph, text_area);
        }
        {
            let tableau_area = areas[1];

            // Surround the area with a border
            frame.render_widget(Block::bordered(), tableau_area);
            let tableau_area = tableau_area.inner(Margin {
                horizontal: 2,
                vertical: 1,
            });

            frame.render_stateful_widget(
                TableauWidget::new(self.cursor),
                tableau_area,
                &mut self.game_engine.tableau(),
            )
        }
    }
}

fn fill_rect_bg_with(frame: &mut Frame, area: Rect, color: Color) {
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            frame
                .buffer_mut()
                .cell_mut(Position::new(x, y))
                .unwrap()
                .set_bg(color);
        }
    }
}
