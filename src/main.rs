use card_deck::CardDeck;
use core::GameMode;
use crossterm::event::{Event, KeyCode, read};

mod core;
mod data_structures;

mod card_deck;
mod card_pile;
mod card_stock;
mod card_sequence;
mod game_tableau;
mod game;

fn wait_for_cards() {
    println!("Press [ENTER] key to take some cards...");
    loop {
        if let Ok(Event::Key(event)) = read() {
            if event.code == KeyCode::Enter {
                break;
            }
        }
    }
}
fn main() {
    let mut deck = CardDeck::new(GameMode::TwoSuits);

    while !deck.is_empty() {
        println!("Deck has {} cards left", deck.len());
        wait_for_cards();

        let some_cards = deck.take_cards(15);

        println!(
            "Next cards: {}",
            some_cards
                .iter()
                .map(|card| format!("{} ", card.to_string()))
                .collect::<String>()
        );
    }

    println!("We're done!");
}
