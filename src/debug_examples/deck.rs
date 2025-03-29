use std::collections::HashMap;
use std::io::{self, Write};

use crossterm::event::{Event, KeyCode, read};

use crate::{card_deck::CardDeck, core::GameMode};

fn wait_for_game_mode() -> GameMode {
    let mut game_mode_key_codes = HashMap::new();
    game_mode_key_codes.insert(KeyCode::Char('1'), GameMode::OneSuit);
    game_mode_key_codes.insert(KeyCode::Char('2'), GameMode::TwoSuits);
    game_mode_key_codes.insert(KeyCode::Char('4'), GameMode::FourSuits);

    print!("Press <1> for OneSuit, <2> for TwoSuits or <4> for FourSuits: ");
    io::stdout().flush().unwrap();

    loop {
        if let Ok(Event::Key(event)) = read() {
            if let Some(game_mode) = game_mode_key_codes.get(&event.code) {
                return *game_mode;
            }
        }
    }
}

fn wait_for_cards_command() {
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
    let mut deck = CardDeck::new(wait_for_game_mode());
    let _ = read();
    io::stdout().flush().unwrap();

    while !deck.is_empty() {
        println!("Deck has {} cards left", deck.len());
        wait_for_cards_command();

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
