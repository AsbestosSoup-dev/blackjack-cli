use crate::models::card::{Card, Rank, Suit};
use rand::prelude::*;

pub struct Deck {
    cards: Vec<Card>,
    current_position: usize,
    cut_position: usize,
    rng: ThreadRng,
}

impl Deck {
    fn new(num_decks: usize) -> Self {
        let total_cards = num_decks * 52;
        let mut rng = rand::rng();
        let mut cards: Vec<Card> = Vec::with_capacity(total_cards);
        for _ in 0..num_decks {
            for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
                for rank in [Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                    cards.push(Card(rank, suit));
                }
            }
        }
        
        // Maybe allow players to choose penetration values,
        // or define pen values to offer difficulty levels (anti-count-counting)
        let cut_position_low = (total_cards as f32 * 0.7) as usize;
        let cut_position_high = (total_cards as f32 * 0.8) as usize;
        let cut_position = rng.random_range(cut_position_low..=cut_position_high);
        
        Self {
            cards,
            current_position: 0,
            cut_position,
            rng,
        }
    }
    
    fn draw_card(&mut self) -> &Card {
        let card = &self.cards[self.current_position];
        self.current_position += 1;
        card
    }
    
    fn shuffle(&mut self) {
        self.cards.shuffle(&mut self.rng);
        self.current_position = 0;
        self.cut_position = {
            let total_cards = self.cards.len();
            let cut_position_low = (total_cards as f32 * 0.7) as usize;
            let cut_position_high = (total_cards as f32 * 0.8) as usize;
            self.rng.random_range(cut_position_low..=cut_position_high)
        }
    }
}