use anyhow::{Result, Error};
use core::result::Result::Ok;
use crate::models::game::{Game};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;

#[derive(Debug)]
pub struct GameMediator {
    repo: SurrealDBRepo
}

impl GameMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        GameMediator { repo }
    }

    pub async fn get_game(&self, id: String) -> Result<WithId<Game>,Error> {
        self.repo.get_game(id)
            .await?
            .ok_or(Error::msg("not found"))
    }

    pub async fn create_game(&self, creator_id: &str) -> Result<WithId<Game>,Error> {
        println!("creating a new discard pile...");
        let discard = self.repo.create_empty_discard().await?;

        println!("creating a new game...");
        let game = self.repo.create_game(discard.id, creator_id).await?;

        println!("game {} created", game.id.id);
        Ok(game)
    }
}
