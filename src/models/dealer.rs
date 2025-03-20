use crate::models::card::Card;

pub struct Dealer {
    hand: Vec<Card>,
    score: u8,
}

impl Dealer {
    fn deal_card(&mut self) -> &Card {
        unimplemented!()
    }
}