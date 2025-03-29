use crossterm::event::{Event, KeyCode, read};

use crate::{card_deck::CardDeck, core::GameMode};

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

pub fn console_debug_deck() {
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
