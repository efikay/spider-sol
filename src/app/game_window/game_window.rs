#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::game::{card_stock::ICardStock, game_engine::GameEngine};

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
        Self {
            game_engine: GameEngine::new(stock),
            cursor: GameCursor::new(),
        }
    }

    fn deal_cards(&mut self) {
        self.game_engine.deal_cards();
    }

    fn is_selecting_a_card(&self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::CardSelect(_)) => true,
            _ => false,
        }
    }
    fn is_selecting_a_pile(&self) -> bool {
        match self.cursor.mode() {
            Some(GameCursorMode::PileSelect(_)) => true,
            _ => false,
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
            // TODO: Select a card by:
            // - Taking `cursor` position, then
            // - Finding a card in `game_engine`
            // - Save the card

            let pile_filters = self
                .game_engine
                .tableau()
                .borrow()
                .piles()
                .borrow()
                .iter()
                .map(|pile| !pile.is_empty())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            self.cursor.set_for_pile_selection(pile_filters);
        } else if self.is_selecting_a_pile() {
            // TODO: Perform a move by:
            // - Taking `cursor` position, then (one of):
            //   - (OR) Create a `CardMove` based on saved Card, cursor position and pile.is_empty() state
            //   - (OR) Find previously(todo?) saved available move based on such data
            // - Then, perform a move through game_engine

            let playable_card_lengths = self
                .game_engine
                .tableau()
                .borrow()
                .piles()
                .borrow()
                .iter()
                .map(|pile| !pile.playable_cards_len())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            self.cursor.set_for_card_selection(playable_card_lengths);
        }
    }
    fn on_d_pressed(&mut self) {
        if self.is_selecting_a_card() {
            self.deal_cards();
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

    // --- Render --- //
    pub fn render_window(&self, frame: &mut Frame) {
        let areas = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame.area());
        {
            let text_area = areas[0];
            let text = Text::from("Press <d> to deal some cards. Press <q> to exit");

            let paragraph = Paragraph::new(text)
                .block(Block::bordered())
                .style(Style::default());

            frame.render_widget(paragraph, text_area);
        }
        {
            let tableau_area = areas[1];

            // TODO: Edit Tableau+CardPile widgets states to take required cursor data to highlight what's needed

            // Surround the area with a border
            frame.render_widget(Block::bordered(), tableau_area);
            let tableau_area = tableau_area.inner(Margin {
                horizontal: 2,
                vertical: 1,
            });

            frame.render_stateful_widget(
                TableauWidget::default(),
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
