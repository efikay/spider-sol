use crossterm::event::{Event, KeyCode, read};

use crate::game::{
    card_deck::CardDeck,
    card_stock::{CardStock, InitialCards},
    card_stock_trait::ICardStock,
    core::{Card, GameMode, PILES_AMOUNT, Rank, Suit},
    debug_examples::mock_parts::SameCardIncDealStock,
    game_engine::GameEngine,
    game_tableau::GameTableau,
};

pub fn console_debug_deals() {
    let mut game = GameEngine::from_tableau_and_stock(
        GameTableau::new(InitialCards {
            face_down_cards: vec![],
            face_up_cards: vec![
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
                Card::new_opened(Rank::Ace, Suit::Spades),
            ],
        }),
        SameCardIncDealStock::new(Rank::Two),
    );

    println!("Game started (aces start, same card++ deals)");
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
