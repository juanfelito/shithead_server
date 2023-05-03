use serde::{Serialize, Deserialize};
use crate::shithead::Cards as ProtoCards;

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

impl Into<Option<ProtoCards>> for Cards {
    fn into(self) -> Option<ProtoCards> {
        Some(ProtoCards {
            hand: self.hand,
            face_up: self.face_up,
            face_down: self.face_down,
        })
    }
}
