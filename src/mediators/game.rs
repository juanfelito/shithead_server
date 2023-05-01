use crate::repo::SurrealDBRepo;
use anyhow::{Result, Error};
use crate::models::{Game, WithId};
use core::result::Result::Ok;

#[derive(Debug)]
pub struct GameMediator {
    repo: SurrealDBRepo
}

impl GameMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        GameMediator { repo }
    }

    pub async fn get_game(&self, id: String) -> Result<WithId<Game>,Error> {
        println!("trying to get game by id");
        let game: Option<WithId<Game>> = self.repo.db.select(("game", id)).await?;
        match game {
            Some(game) => {
                Ok(game)
            }
            None => {
                Err(Error::msg("not found"))
            }
        }
    }

    pub async fn create_game(&self, creator_id: String) -> Result<WithId<Game>,Error> {
        println!("trying to create a new game");
        let game: Result<WithId<Game>, surrealdb::Error> = self.repo.db.create("game")
            .content(Game{
                players: vec![creator_id]
            })
        .await;
        match game {
            Ok(game) => {
                Ok(game)
            }
            Err(_) => {
                Err(Error::msg("couldn't create new game"))
            }
        }
    }
}