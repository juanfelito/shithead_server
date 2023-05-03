use serde::{Serialize, Deserialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum GameState {
    Lobby,
    Started,
    Finished,
}

impl Into<String> for GameState {
    fn into(self) -> String {
        match self {
            Self::Lobby => "Lobby".to_string(),
            Self::Started => "Started".to_string(),
            Self::Finished => "Finished".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub creator: Thing,
    pub deck: Vec<String>,
    pub discard: Thing,
    pub players_out: Vec<u32>,
    pub state: GameState,
    pub turn: u32,
    pub users: Option<Vec<Thing>>,
}
