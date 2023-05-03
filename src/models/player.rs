use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub turn: u32,
    pub cards: Cards,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cards {
    pub hand: Vec<String>,
    pub face_up: Vec<String>,
    pub face_down: Vec<String>,
}
