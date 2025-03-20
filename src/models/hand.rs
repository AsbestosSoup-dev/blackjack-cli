use crate::models::card::Card;

pub struct Hand {
    cards: Vec<Card>,
    bet: u64,
    is_doubled: bool,
    is_bust: bool,
    score: Option<u8>,
}

impl Hand {
    fn split(&mut self) {
        unimplemented!()
    }
}