#![allow(dead_code)]

use crossterm::event::{Event, KeyCode, read};

use crate::game::{card_deck::CardDeck, card_stock::CardStock, core::GameMode, game_engine::GameEngine};

pub fn console_debug_game_start_and_deals() {
    let stock = CardStock::new(CardDeck::new(GameMode::FourSuits));
    let mut game = GameEngine::new(stock);

    println!("Game started, FourSuits mode");
    while game.deals_left() > 0 {
        println!("{}", game);

        println!("Press [Enter] to deal more cards...");
        if let Ok(Event::Key(event)) = read() {
            if event.code == KeyCode::Enter {
                game.deal_cards();
                continue;
            }
        }
    }

    println!("{}", game);
    println!("Game finished!");
}
