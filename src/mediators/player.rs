use anyhow::{Result, Error};
use core::result::Result::Ok;
use crate::models::game::{Game, GameState};
use crate::models::player::{Player};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;

#[derive(Debug)]
pub struct PlayerMediator {
    repo: SurrealDBRepo
}

impl PlayerMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        PlayerMediator { repo }
    }

    pub async fn join_game(&self, game_id: String, user_id: String) -> Result<WithId<Player>, Error> {
        println!("verifying before joining game...");
        let game_opt: Option<WithId<Game>> = self.repo.db.select(("game", &game_id)).await?;
        if game_opt.is_none() {
            Err::<WithId<Player>, anyhow::Error>(Error::msg("game not found"));
        }
        let game: WithId<Game> = game_opt.unwrap();

        if game.inner.state != GameState::Lobby {
            Err::<WithId<Player>, anyhow::Error>(Error::msg("game already started"));
        }

        // TODO: check if user is already in game and how many are in to set turn
        let turn = 0;
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
