#![allow(dead_code)]

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::game::{card_stock::ICardStock, game_engine::GameEngine};

use super::widgets::TableauWidget;

pub struct GameWindow<CardStockT: ICardStock> {
    game_engine: GameEngine<CardStockT>,
}

impl<CardStockT: ICardStock> GameWindow<CardStockT> {
    pub fn new(stock: CardStockT) -> GameWindow<CardStockT> {
        let game_engine = GameEngine::new(stock);

        Self { game_engine }
    }

    pub fn deal_cards(&mut self) {
        self.game_engine.deal_cards();
    }

    pub fn render_window(&self, frame: &mut Frame) {
        let areas = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame.area());
        {
            let text_area = areas[0];
            let text = Text::from("Press <Enter> to deal some cards. Press <q> to exit");

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
