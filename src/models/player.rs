use serde::{Serialize, Deserialize};
use surrealdb::sql::{Thing, Id};
use crate::shithead::Cards as ProtoCards;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub turn: u32,
    pub cards: Cards,
    pub r#in: Thing,
}

impl Default for Player {
    fn default() -> Self {
        Self { turn: 0, cards: Cards::default(), r#in: Thing { tb: "".to_string(), id: "".to_string().into() } }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
