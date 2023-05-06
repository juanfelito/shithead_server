use anyhow::{anyhow, Error};
use crate::repo::SurrealDBRepo;
use crate::models::WithId;
use crate::models::discard::{Discard};

use super::MediatorError;

#[derive(Debug)]
pub struct DiscardMediator {
    repo: SurrealDBRepo
}

impl DiscardMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        DiscardMediator { repo }
    }

    pub async fn get_discard(&self, game_id: String) -> Result<WithId<Discard>,Error> {
        self.repo.get_discard(game_id)
                .await?
                .ok_or(anyhow!(MediatorError::NotFound("Discard not found".to_string())))
    }
}