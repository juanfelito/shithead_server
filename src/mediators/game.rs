use anyhow::{Result, Error};
use surrealdb::sql::Thing;
use core::result::Result::Ok;
use crate::models::discard::Discard;
use crate::models::game::{Game, GameState};
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
        println!("trying to get game by id");
        let sql = format!("select *, <-player<-user as users from game:{}", &id);
        let mut result = self.repo.db.query(sql).await?;
        let game: Option<WithId<Game>> = result.take(0)?;
        match game {
            Some(game) => {
                Ok(game)
            }
            None => {
                Err(Error::msg("not found"))
            }
        }
    }

    pub async fn create_game(&self, creator_id: &str) -> Result<WithId<Game>,Error> {
        println!("creating a new discard pile...");
        let discard: WithId<Discard> = self.repo.db.create("discard")
            .content(Discard{
                current_value: None,
                current_card: None,
                repeat_count: 0,
                cards: vec![],
            })
        .await.expect("couldn't create empty discard");

        println!("creating a new game...");
        let game: Result<WithId<Game>, surrealdb::Error> = self.repo.db.create("game")
            .content(Game{
                creator: Thing::from(("user", creator_id)),
                deck: vec![],
                discard: discard.id,
                players_out: vec![],
                state: GameState::Lobby,
                turn: 0,
                users: None,
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
