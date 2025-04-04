use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::Paragraph,
};

use crate::game::{card_stock::ICardStock, game_engine::GameEngine};

use super::widgets::TableauWidget;

#[allow(dead_code)]

pub struct GameWindow<CardStockT: ICardStock> {
    game_engine: GameEngine<CardStockT>,
}

impl<CardStockT: ICardStock> GameWindow<CardStockT> {
    pub fn new(stock: CardStockT) -> GameWindow<CardStockT> {
        let game_engine = GameEngine::new(stock);

        Self { game_engine }
    }

    pub fn render_window(&self, frame: &mut Frame) {
        let areas = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame.area());
        {
            let title_area = areas[0];
            let title = Line::from("Spider solitaire").bold().blue().centered();

            frame.render_widget(Paragraph::new(title).centered(), title_area);
        }
        {
            let tableau_area = areas[1];

            frame.render_stateful_widget(
                TableauWidget::default(),
                tableau_area,
                &mut self.game_engine.tableau(),
            )
        }
    }
}
