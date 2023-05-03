use anyhow::{Result, Error};
use core::result::Result::Ok;
use crate::models::game::{Game, GameState};
use crate::models::player::{Player};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;

#[derive(Debug, Clone)]
pub struct PlayerMediator {
    repo: SurrealDBRepo
}

impl PlayerMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        PlayerMediator { repo }
    }

    pub async fn join_game(&self, game_id: String, user_id: String) -> Result<WithId<Player>, Error> {
        println!("verifying before joining game...");
        let sql = format!("select *, <-player<-user as users from game:{}", &game_id);
        let mut result = self.repo.db.query(sql).await?;
        let game_opt: Option<WithId<Game>> = result.take(0)?;
        if game_opt.is_none() {
            return Err(Error::msg("game not found"));
        }
        let game: WithId<Game> = game_opt.unwrap();

        let users = game.inner.users.unwrap_or_default();
        let already_joined = users.iter().any(|u| u.id.to_string() == user_id);

        if already_joined {
            return Err(Error::msg("already joined this game"));
        }
        if game.inner.state != GameState::Lobby {
            return Err(Error::msg("game already started"));
        }

        let turn = users.len();
        println!("joining game...");

        let sql = format!("relate user:{}->player->game:{} content {{turn: {}, cards: {{hand: [], face_up: [], face_down: []}}}}", user_id, game_id, turn);
        let mut result = self.repo.db.query(sql).await?;
        let player: Option<WithId<Player>> = result.take(0)?;
        
        match player {
            Some(player) => {
                Ok(player)
            }
            None => {
                Err(Error::msg("not found"))
            }
        }

    }
}
