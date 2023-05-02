use crate::models::WithId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Discard {
    pub cards: Vec<String>,
    pub current_value: Option<String>,
    pub current_card: Option<String>,
    pub repeat_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WrappedDiscard {
    pub discard: WithId<Discard>
}
