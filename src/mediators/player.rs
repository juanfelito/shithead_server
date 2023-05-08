use anyhow::{anyhow, Result, Error, Ok};
use crate::dealer::{Dealer};
use crate::models::game::{Game, GameState};
use crate::models::player::{Player};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;
use super::MediatorError;

#[derive(Debug)]
pub struct PlayerMediator {
    repo: SurrealDBRepo,
    dealer: Dealer
}

impl PlayerMediator {
    pub fn new(repo: SurrealDBRepo, dealer: Dealer) -> Self {
        PlayerMediator { repo, dealer }
    }

    pub async fn play(&self, player_id: String, chosen_cards: Vec<String>) -> Result<(), Error> {
        let (player, game) = self.repo.get_player_and_game(player_id).await?;

        let mut player = player.ok_or(anyhow!(MediatorError::NotFound("Player not found".to_string())))?;
        let game = game.ok_or(anyhow!(MediatorError::NotFound("Game not found".to_string())))?;

        if game.inner.turn != player.inner.turn {
            return Err(anyhow!(MediatorError::Unavailable("It's not this player's turn".to_string())));
        }

        let cards = self.dealer.get_active_cards(&mut player.inner);

        println!("cards before: {:?}", cards);

        let mut cards_to_check = vec![];
        for chosen_card in &chosen_cards {
            for (index, card) in cards.iter().enumerate() {
                if card == chosen_card {
                    cards_to_check.push(cards.remove(index));
                    break;
                }
            }
        }

        if cards_to_check.len() != chosen_cards.len() {
            return Err(anyhow!(MediatorError::InvalidArgument("The player doesn't have one or more of the cards sent, or they are not active in this turn".to_string())));
        }
        
        println!("cards after: {:?}", cards);
        println!("cards from hand that matched: {:?}", cards_to_check);

        Ok(())
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
