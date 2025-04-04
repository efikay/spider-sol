#![allow(dead_code)]

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

use crate::game::{card_stock::CardDeckStock, core::GameMode};

use super::game_window::GameWindow;

pub struct App {
    is_running: bool,
    game_window: GameWindow<CardDeckStock>,
}

impl App {
    pub fn new() -> App {
        Self {
            is_running: false,
            game_window: GameWindow::new(CardDeckStock::new(GameMode::TwoSuits)),
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

    fn render(&mut self, frame: &mut Frame) {
        self.game_window.render_window(frame)
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
        match (key.modifiers, key.code) {
            // [Quit combinations]
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // [Arrow navigation]
            (_, KeyCode::Left | KeyCode::Char('h')) => self.on_left_pressed(),
            (_, KeyCode::Down | KeyCode::Char('j')) => self.on_down_pressed(),
            (_, KeyCode::Up | KeyCode::Char('k')) => self.on_up_pressed(),
            (_, KeyCode::Right | KeyCode::Char('l')) => self.on_right_pressed(),
            // Add other key handlers here
            _ => {}
        }
    }

    fn on_up_pressed(&self) {
        todo!("TODO: [⤴]")
    }
    fn on_down_pressed(&self) {
        todo!("TODO: [↓]")
    }
    fn on_left_pressed(&self) {
        todo!("TODO: [←]")
    }
    fn on_right_pressed(&self) {
        todo!("TODO: [→]")
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}
