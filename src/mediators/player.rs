use anyhow::{anyhow, Result, Error};
use crate::models::game::{Game, GameState};
use crate::models::player::{Player};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;
use super::MediatorError;

#[derive(Debug, Clone)]
pub struct PlayerMediator {
    repo: SurrealDBRepo
}

impl PlayerMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        PlayerMediator { repo }
    }

    pub async fn join_game(&self, game_id: String, user_id: String) -> Result<WithId<Player>, Error> {
        let game: WithId<Game> = self.repo.get_game(game_id.clone())
                                        .await?
                                        .ok_or(anyhow!(MediatorError::NotFound("Game not found".to_string())))?;

        let users = game.inner.users.unwrap_or_default();
        let already_joined = users.iter().any(|u| u.id.to_string() == user_id);

        if already_joined {
            return Err(anyhow!(MediatorError::AlreadyExists("Already joined this game".to_string())));
        }
        if game.inner.state != GameState::Lobby {
            return Err(anyhow!(MediatorError::Unavailable("Game already started".to_string())));
        }

        let turn = users.len();
        println!("joining game...");

        self.repo.join_game(game_id, user_id, turn)
                .await?
                .ok_or(anyhow!(MediatorError::Internal("Couldn't join the game".to_string())))
    }

    pub async fn get_player(&self, game_id: String, user_id: String) -> Result<WithId<Player>, Error> {
        self.repo.get_player(game_id, user_id)
                .await?
                .ok_or(anyhow!(MediatorError::NotFound("Player not found".to_string())))
    }
}
