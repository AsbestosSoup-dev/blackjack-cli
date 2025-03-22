use std::ffi::c_ushort;
use std::io::{stdin, stdout, Write};
use serde::{Serialize, Deserialize};

use crate::models::player::Player;
use crate::models::deck::Deck;
use crate::models::dealer::Dealer;

struct Game {
    phase: GamePhase,
    deck: Deck,
    dealer: Dealer,
    players: Vec<Player>,
    config: GameConfig,
}

impl Game {
    fn config(&mut self, config: Option<GameConfig>) {
        if let Some(config) = config {
            self.config = config;
        } else {
            let mut input = String::new();

            // Interactive mode
            loop {
                print!("How many players? (max. 4): ");
                stdout().flush().unwrap();
                if let Ok(num_players) = stdin().read_line(&mut input) {
                    println!("{num_players}");
                    // if (1..=4).contains(num_players.) {
                    //
                    // }
                }
            }

        }
    }
}

#[derive(Debug, Copy, Clone)]
struct GameConfig {
    blackjack_payout: f32,
    min_bet: u32,
    max_bet: u32,
    num_players: u8,
    num_decks: u8,
    dealer_stands_soft_17: bool,
    allow_insurance: bool,
    allow_double_on_split: bool,
    max_splits_per_hand: u8,
}

enum GamePhase {
    Init,
    Dealing,
    Insurance,
    Betting,
    PlayerTurns,
    DealerTurn,
    Payout,
}