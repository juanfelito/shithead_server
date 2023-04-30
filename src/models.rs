use surrealdb::sql::{Thing};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WithId<T> {
    pub id: Thing,
    #[serde(flatten)]
    pub inner: T,
}

impl<T> WithId<T> {
    fn new(id: Thing, inner: T) -> Self {
        Self { id, inner }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub players: Vec<String>,
}
