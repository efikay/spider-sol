#![allow(dead_code)]

use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, read};

use crate::game::{
    card_stock::InitialCards,
    core::{Card, Rank, Suit},
    debug_examples::{helpers::read_stdin_as_i32, mock_parts::CardIncCycleInfiniteStock},
    game_engine::GameEngine,
    game_tableau::GameTableau,
};

pub fn console_debug_moves() {
    let mut game = GameEngine::from_tableau_and_stock(
        GameTableau::new(InitialCards {
            face_down_cards: vec![],
            face_up_cards: vec![
                Card::new_opened(Rank::Ace, Suit::Hearts),
                Card::new_opened(Rank::Two, Suit::Hearts),
                Card::new_opened(Rank::Three, Suit::Hearts),
                Card::new_opened(Rank::Four, Suit::Hearts),
                Card::new_opened(Rank::Five, Suit::Hearts),
                Card::new_opened(Rank::Six, Suit::Hearts),
                Card::new_opened(Rank::Seven, Suit::Hearts),
                Card::new_opened(Rank::Eight, Suit::Hearts),
                Card::new_opened(Rank::Nine, Suit::Hearts),
                Card::new_opened(Rank::Ten, Suit::Hearts),
                Card::new_opened(Rank::Jack, Suit::Hearts),
                Card::new_opened(Rank::Queen, Suit::Hearts),
                Card::new_opened(Rank::King, Suit::Hearts),
            ],
        }),
        CardIncCycleInfiniteStock::new(Rank::Two),
    );

    println!("Game started (game_moves, card+++ deals)");
    while game.deals_left() > 0 {
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", game);

        let moves = game.get_available_moves();
        println!("Available moves (enter number):");
        for (index, mv) in moves.iter().enumerate() {
            println!("{}. {}", index + 1, mv);
        }

        let number_result = read_stdin_as_i32().unwrap_or(1);
        match moves.get((number_result - 1) as usize) {
            Some(card_move) => {
                println!("Card move selected: {}", card_move);
                print!("Performing card move... ");

                match game.perform_move(*card_move) {
                    Ok(_) => println!("– OK!"),
                    Err(_) => panic!("Some error occurred x("),
                }

                game.search_and_update_complete_sequences();
            }
            _ => {
                panic!("No such move :((");
            }
        };
    }

    println!("{}", game);
    println!("Game finished!");
}
