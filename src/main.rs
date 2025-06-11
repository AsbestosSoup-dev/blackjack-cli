use rand::rng;
use rand::seq::SliceRandom;
use ratatui::crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};
use std::fmt;

fn main() {
    println!("Starting Blackjack CLI");

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(draw).expect("Failed to draw frame");
        if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
            break;
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello Blackjack");
    frame.render_widget(text, frame.area());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn symbol(&self) -> &'static str {
        match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn symbol(&self) -> &'static str {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "A",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank.symbol(), self.suit.symbol())
    }
}

struct Deck {
    cards: Vec<Card>,
    num_decks: usize,
    cut_position: usize,
    top_index: usize,
}

impl Deck {
    fn new(num_decks: usize, penetration: f64) -> Result<Self, DeckError> {
        if num_decks == 0 || num_decks > 8 {
            return Err(DeckError::InvalidNumDecks {num_decks});
        }

        if !(0.0..=1.0).contains(&penetration) {
            return Err(DeckError::InvalidPenetration {penetration});
        }

        let num_cards = (num_decks * 52) as f64;
        let cut_position = (num_cards * penetration).floor() as usize;

        Ok(Deck {
            cards: Self::build_deck(num_decks),
            num_decks,
            cut_position, // Todo: loop cut_position - 1 times vs checking condition every cycle
            top_index: 0
        })
    }

    fn new_single() -> Vec<Card> {
        let mut deck = Vec::with_capacity(52);
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace
        ];

        for &suit in suits.iter() {
            for &rank in ranks.iter() {
                deck.push(Card{rank, suit });
            }
        }
        deck
    }

    fn build_deck(num_decks: usize) -> Vec<Card> {
        let single_deck = Deck::new_single();
        let mut compound_deck = Vec::with_capacity(num_decks * 52);
        for _ in 0..num_decks {
            compound_deck.extend(single_deck.iter().copied());
        }
        compound_deck
    }

    fn draw(&mut self) -> Card {
        if self.top_index >= self.cut_position {
            self.shuffle();
        }
        
        let drawn_card = self.cards[self.top_index];
        self.top_index += 1;
        drawn_card
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
        self.top_index = 0;
    }
}

#[derive(Debug)]
enum DeckError {
    InvalidNumDecks {num_decks: usize},
    InvalidPenetration {penetration: f64},
}

impl fmt::Display for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeckError::InvalidNumDecks {num_decks} => write!(
                f,
                "Number of decks must be between 1 and 8. Entered: {num_decks}"
            ),
            DeckError::InvalidPenetration {penetration} => write!(
                f,
                "Penetration must be between 0.0 and 1.0. Entered: {:.3}",
                penetration
            )
        }
    }
}