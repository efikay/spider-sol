#![allow(dead_code)]

use crossterm::event::{Event, KeyCode, read};

use crate::game::{
    card_deck::CardDeck,
    card_stock::{CardDeckStock, mocks::SameCardDecDealStock},
    core::{Card, GameMode, PILES_AMOUNT, Rank, Suit},
    game_engine::GameEngine,
    game_tableau::GameTableau,
};

pub fn console_debug_deals() {
    let mut game = GameEngine::new(SameCardDecDealStock::new(Rank::Queen));

    println!("Game started (aces start, same card++ deals)");
    while game.deals_left() > 0 {
        println!("{}", game);

        println!("Press [Enter] to deal more cards...");
        if let Ok(Event::Key(event)) = read() {
            if event.code == KeyCode::Enter {
                game.deal_cards();
                game.search_and_update_complete_sequences();
                continue;
            }
        }
    }

    println!("{}", game);
    println!("Game finished!");
}
