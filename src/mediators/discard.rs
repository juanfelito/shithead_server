use anyhow::Error;
use crate::repo::SurrealDBRepo;
use crate::models::WithId;
use crate::models::discard::{Discard, WrappedDiscard};

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

        let sql = format!("select discard from game:{} fetch discard", game_id);
        let mut result = self.repo.db.query(sql).await?;
        let discard: Option<WrappedDiscard> = result.take(0)?;
        match discard {
            Some(w_discard) => {
                Ok(w_discard.discard)
            }
            None => {
                Err(Error::msg("not found"))
            }
        }
    }
}