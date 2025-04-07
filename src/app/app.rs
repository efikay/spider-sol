#![allow(dead_code)]

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::game::{card_stock::CardDeckStock, core::GameMode};

use super::{
    game_window::{GameWindow, GameWindowKeyResult},
    welcome_window::{WelcomeWindow, WelcomeWindowKeyResult},
};

pub struct App {
    is_running: bool,

    game_window: Option<GameWindow<CardDeckStock>>,
    welcome_window: Option<WelcomeWindow>,
}

impl App {
    pub fn new() -> App {
        Self {
            is_running: false,
            game_window: None,
            welcome_window: Some(WelcomeWindow::new()),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.is_running = true;

        while self.is_running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }

        Ok(())
    }

    fn start_new_game(&mut self, game_mode: GameMode) {
        self.welcome_window = None;
        self.game_window = Some(GameWindow::new(CardDeckStock::new(game_mode)));
    }
    fn stop_the_game(&mut self) {
        self.game_window = None;
        self.welcome_window = Some(WelcomeWindow::new());
    }

    fn render(&mut self, frame: &mut Frame) {
        if let Some(window) = &self.welcome_window {
            window.render_window(frame)
        } else if let Some(window) = self.game_window.as_mut() {
            window.render_window(frame)
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if let Some(welcome_window) = self.welcome_window.as_mut() {
            if let Some(key_result) = welcome_window.on_key_pressed(key) {
                match key_result {
                    WelcomeWindowKeyResult::NewGame(game_mode) => {
                        return self.start_new_game(game_mode);
                    }
                    WelcomeWindowKeyResult::ExitGame => {
                        return self.quit();
                    }
                }
            }
        } else if let Some(game_window) = self.game_window.as_mut() {
            if let Some(key_result) = game_window.on_key_pressed(key) {
                match key_result {
                    GameWindowKeyResult::StopTheGame => {
                        return self.stop_the_game();
                    }
                }
            }
        }
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}
