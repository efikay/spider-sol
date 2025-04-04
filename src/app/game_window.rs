use ratatui::{
    Frame,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::game::{card_stock::ICardStock, game_engine::GameEngine};

#[allow(dead_code)]

pub struct GameWindow<CardStockT: ICardStock> {
    game_engine: GameEngine<CardStockT>,
}

impl<CardStockT: ICardStock> GameWindow<CardStockT> {
    pub fn new(stock: CardStockT) -> GameWindow<CardStockT> {
        Self {
            game_engine: GameEngine::new(stock),
        }
    }

    pub fn render_window(&self, frame: &mut Frame) {
        let title = Line::from("Spider solitaire").bold().blue().centered();
        let text = "GAME_WINDOW";

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }
}
