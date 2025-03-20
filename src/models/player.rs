use uuid::Uuid;
use crate::models::hand::Hand;

pub struct Player {
    uuid: Uuid,
    name: String,
    hands: Vec<Hand>,
    bank: usize,
    insurance: usize,
    score: usize,
}

pub enum PlayerAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Insurance,
}