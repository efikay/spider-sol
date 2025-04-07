#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Modifier, Style},
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

pub enum GameWindowKeyResult {
    // User wants to stop the game (go back to welcome screen)
    StopTheGame = 1,
    // User wants to restart the game (with the same mode)
    RestartTheGame,
}
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

    fn has_empty_piles(&self) -> bool {
        let tableau = self.game_engine.tableau();
        let tableau = tableau.borrow();

        let piles = tableau.piles();
        let piles = piles.borrow();

        piles.iter().any(|p| p.is_empty())
    }

    fn deals_left(&self) -> usize {
        self.game_engine.deals_left()
    }
    fn can_deal_cards(&self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::CardSelect(_)) => self.deals_left() > 0 && !self.has_empty_piles(),
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
    pub fn on_key_pressed(&mut self, key: KeyEvent) -> Option<GameWindowKeyResult> {
        match (key.modifiers, key.code) {
            // [Stop the game]
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                return Some(GameWindowKeyResult::StopTheGame);
            }
            // [Arrow navigation]
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a')) => self.on_left_pressed(),
            (_, KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s')) => self.on_down_pressed(),
            (_, KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w')) => self.on_up_pressed(),
            (_, KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d')) => {
                self.on_right_pressed()
            }
            // [Deal cards]
            (_, KeyCode::Char(' ')) => self.on_d_pressed(),
            // [Restart the game]
            (_, KeyCode::Char('r')) => return Some(GameWindowKeyResult::RestartTheGame),
            // [Select a card / Select a pile]
            (_, KeyCode::Enter | KeyCode::Tab) => self.on_action_pressed(),
            _ => {
                // None
            }
        };

        None
    }
    fn on_action_pressed(&mut self) {
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
        if !self.is_placing_a_card() && self.deals_left() > 0 {
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

        if self.game_engine.is_won() {
            return frame.render_widget(
                Paragraph::new("YOU WIN! PRESS <Q> TO exit to menu"),
                frame.area(),
            );
        }

        let areas = Layout::vertical([Constraint::Min(4), Constraint::Percentage(100)])
            .split(frame.area());

        {
            let text_area = areas[0];

            let paragraph = Paragraph::new(self.make_info_text())
                .block(Block::bordered())
                .style(Style::default());

            frame.render_widget(paragraph, text_area);
        }
        {
            let tableau_area = areas[1];

            // Surround the area with a border
            let border_block = if self.is_placing_a_card() {
                Block::bordered().border_style(Style::new().add_modifier(Modifier::BOLD))
            } else {
                Block::bordered()
            };

            frame.render_widget(border_block, tableau_area);
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

    fn make_info_text(&self) -> Text {
        let deals_left_text = format!("Deals left: {}", self.deals_left());
        let deal_hint = if self.can_deal_cards() {
            "<tab|enter> – deal"
        } else {
            ".."
        };
        let restart_hint = "<r> – restart";
        let exit_hint = "<q> – menu";
        let sequences_text = format!(
            "Complete sequences: - {}/8",
            self.game_engine.complete_sequences_count()
        );
        let navigation_hint = "wasd, hjkl, ←↑↓→ - navigation";

        Text::from(vec![
            Line::from(format!(
                "{} | {} | {} | {}",
                exit_hint, restart_hint, deal_hint, navigation_hint
            )),
            Line::from(format!("{} | {}", deals_left_text, sequences_text)),
        ])
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
