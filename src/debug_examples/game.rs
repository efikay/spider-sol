#![allow(dead_code)]

use crossterm::event::{Event, KeyCode, read};

use crate::{card_deck::CardDeck, core::GameMode, game::Game};

pub fn console_debug_game_start_and_deals() {
    let mut game = Game::new(CardDeck::new(GameMode::FourSuits));

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
