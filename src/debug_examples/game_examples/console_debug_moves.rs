#![allow(dead_code)]

use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, read};

use crate::{
    debug_examples::helpers::read_stdin_as_i32,
    game::{
        card_stock::mocks::CardIncCycleInfiniteStock,
        core::{Card, Rank, Suit},
        game_engine::GameEngine,
        game_tableau::GameTableau,
    },
};

pub fn console_debug_moves() {
    let mut game = GameEngine::new(CardIncCycleInfiniteStock::new(Rank::Two));

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
