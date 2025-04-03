#![allow(dead_code)]

use crossterm::event::{Event, KeyCode, read};

use crate::{
    debug_examples::mock_parts::SameCardDecDealStock,
    game::{
        card_deck::CardDeck,
        card_stock::{CardStock, InitialCards},
        card_stock_trait::ICardStock,
        core::{Card, GameMode, PILES_AMOUNT, Rank, Suit},
        game_engine::GameEngine,
        game_tableau::GameTableau,
    },
};

pub fn console_debug_deals() {
    let mut game = GameEngine::from_tableau_and_stock(
        GameTableau::new(InitialCards {
            face_down_cards: vec![],
            face_up_cards: vec![
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
                Card::new_opened(Rank::King, Suit::Spades),
            ],
        }),
        SameCardDecDealStock::new(Rank::Queen),
    );

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
