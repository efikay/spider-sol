#![allow(dead_code)]

use std::fmt;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Stylize},
    text::Text,
    widgets::{Block, Borders, List},
};

use crate::game::core::GameMode;

#[derive(Debug, Clone, Copy)]
pub enum WelcomeWindowKeyResult {
    // User selected new game with game mode
    NewGame(GameMode),
    // User wants window to be closed (exit the game)
    ExitGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WelcomeWindowCursorPosition {
    NewGame(GameMode),
    ExitGame,
}
impl fmt::Display for WelcomeWindowCursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match self {
            WelcomeWindowCursorPosition::NewGame(mode) => format!("New game: {}", mode),
            WelcomeWindowCursorPosition::ExitGame => "Exit game".to_string(),
        };

        write!(f, "{}", as_str)
    }
}
impl WelcomeWindowCursorPosition {
    pub fn next(&self) -> Self {
        match self {
            WelcomeWindowCursorPosition::NewGame(game_mode) => match game_mode {
                GameMode::OneSuit => WelcomeWindowCursorPosition::NewGame(GameMode::TwoSuits),
                GameMode::TwoSuits => WelcomeWindowCursorPosition::NewGame(GameMode::FourSuits),
                GameMode::FourSuits => WelcomeWindowCursorPosition::ExitGame,
            },
            WelcomeWindowCursorPosition::ExitGame => {
                WelcomeWindowCursorPosition::NewGame(GameMode::OneSuit)
            }
        }
    }
    pub fn prev(&self) -> Self {
        match self {
            WelcomeWindowCursorPosition::NewGame(game_mode) => match game_mode {
                GameMode::OneSuit => WelcomeWindowCursorPosition::ExitGame,
                GameMode::TwoSuits => WelcomeWindowCursorPosition::NewGame(GameMode::OneSuit),
                GameMode::FourSuits => WelcomeWindowCursorPosition::NewGame(GameMode::TwoSuits),
            },
            WelcomeWindowCursorPosition::ExitGame => {
                WelcomeWindowCursorPosition::NewGame(GameMode::FourSuits)
            }
        }
    }
}

pub struct WelcomeWindow {
    cursor_position: WelcomeWindowCursorPosition,
}

impl WelcomeWindow {
    pub fn new() -> Self {
        Self {
            cursor_position: WelcomeWindowCursorPosition::NewGame(GameMode::OneSuit),
        }
    }

    // -- Keys -- //
    pub fn on_key_pressed(&mut self, key: KeyEvent) -> Option<WelcomeWindowKeyResult> {
        match (key.modifiers, key.code) {
            // [Arrow navigation]
            (_, KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l')) => None,
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                self.on_down_pressed();

                None
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                self.on_up_pressed();

                None
            }
            // [Select game mode / Select exit]
            (_, KeyCode::Enter) => Some(self.on_enter_pressed()),
            _ => None,
        }
    }
    fn on_down_pressed(&mut self) {
        self.cursor_position = self.cursor_position.next()
    }
    fn on_up_pressed(&mut self) {
        self.cursor_position = self.cursor_position.prev()
    }
    fn on_enter_pressed(&self) -> WelcomeWindowKeyResult {
        match self.cursor_position {
            WelcomeWindowCursorPosition::NewGame(game_mode) => {
                WelcomeWindowKeyResult::NewGame(game_mode)
            }
            WelcomeWindowCursorPosition::ExitGame => WelcomeWindowKeyResult::ExitGame,
        }
    }

    // --- Render --- //
    pub fn render_window(&self, frame: &mut Frame) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(frame.area());
        let _top_margin_area = areas[0];
        let content_area = areas[1];
        let _bottom_margin_area = areas[2];

        let list_chunk = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60), // Center
                Constraint::Percentage(20),
            ])
            .split(content_area)[1];

        let one_suit_text =
            self.get_option_text(WelcomeWindowCursorPosition::NewGame(GameMode::OneSuit));
        let two_suits_text =
            self.get_option_text(WelcomeWindowCursorPosition::NewGame(GameMode::TwoSuits));
        let four_suits_text =
            self.get_option_text(WelcomeWindowCursorPosition::NewGame(GameMode::FourSuits));
        let exit_text = self.get_option_text(WelcomeWindowCursorPosition::ExitGame);

        let list = List::new(vec![
            one_suit_text,
            two_suits_text,
            four_suits_text,
            exit_text,
        ])
        .block(
            Block::default()
                .title("Spider solitaire")
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center),
        );

        // Render the list in the centered area
        frame.render_widget(list, list_chunk);
    }

    fn get_option_text(&self, option: WelcomeWindowCursorPosition) -> Text {
        let as_str = option.to_string();

        match option == self.cursor_position {
            true => Text::from(format!("→ {}", as_str)).add_modifier(Modifier::BOLD),
            false => Text::from(format!("  {}", as_str)),
        }
    }
}
