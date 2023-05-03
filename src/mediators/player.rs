use anyhow::{Result, Error};
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
        let game_opt: Option<WithId<Game>> = self.repo.get_game(game_id.clone()).await?;
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

        self.repo.join_game(game_id, user_id, turn)
                .await?
                .ok_or(Error::msg("couldn't join game"))
    }

    pub async fn get_player(&self, game_id: String, user_id: String) -> Result<WithId<Player>, Error> {
        self.repo.get_player(game_id, user_id)
                .await?
                .ok_or(Error::msg("player not found"))
    }
}
