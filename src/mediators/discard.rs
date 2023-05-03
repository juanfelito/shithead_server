use anyhow::Error;
use crate::repo::SurrealDBRepo;
use crate::models::WithId;
use crate::models::discard::{Discard};

#[derive(Debug)]
pub struct DiscardMediator {
    repo: SurrealDBRepo
}

impl DiscardMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        DiscardMediator { repo }
    }

    pub async fn get_discard(&self, game_id: String) -> Result<WithId<Discard>,Error> {
        println!("trying to get discard by game id");

        self.repo.get_discard(game_id)
                .await?
                .ok_or(Error::msg("not found"))
    }
}