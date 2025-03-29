use debug_examples::console_debug_game_start_and_deals;

mod core;
mod data_structures;

mod card_deck;
mod card_pile;
mod card_sequence;
mod card_stock;
mod debug_examples;
mod game;
mod game_tableau;
mod available_move;
mod card_stock_trait;

fn main() {
    console_debug_game_start_and_deals();
}
