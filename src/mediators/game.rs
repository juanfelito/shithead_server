use anyhow::{anyhow, Result, Error};
use core::result::Result::Ok;
use crate::card_manager::CardManager;
use crate::dealer::Dealer;
use crate::models::game::{Game, GameState};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;

use super::MediatorError;

#[derive(Debug)]
pub struct GameMediator {
    repo: SurrealDBRepo,
    card_manager: CardManager,
    dealer: Dealer
}

impl GameMediator {
    pub fn new(repo: SurrealDBRepo, card_manager: CardManager, dealer: Dealer) -> Self {
        GameMediator { repo, card_manager, dealer }
    }

    pub async fn get_game(&self, id: String) -> Result<WithId<Game>, Error> {
        self.repo.get_game(id)
            .await?
            .ok_or(anyhow!(MediatorError::NotFound("Not found".to_string())))
    }

    pub async fn create_game(&self, creator_id: &str) -> Result<WithId<Game>, Error> {
        println!("creating a new discard pile...");
        let discard = self.repo.create_empty_discard().await?;

        println!("creating a new game...");
        let game = self.repo.create_game(discard.id, creator_id).await?;

        println!("joining game...");
        self.repo.join_game(game.id.id.to_string(), creator_id.to_owned(), 0).await?;

        println!("game {} created", game.id.id);
        Ok(game)
    }

    pub async fn start_game(&self, user_id: String, game_id: String) -> Result<WithId<Game>, Error> {
        let mut game: WithId<Game> = self.repo.get_game(game_id.clone())
                                        .await?
                                        .ok_or(anyhow!(MediatorError::NotFound("Game not found".to_string())))?;
        
        if game.inner.creator.id.to_string() != user_id {
            return Err(anyhow!(MediatorError::Unauthorized("Only the creator can start the game".to_string())));
        }

        if game.inner.state != GameState::Lobby {
            return Err(anyhow!(MediatorError::AlreadyExists("Game already started".to_string())));
        }

        game.inner.users.as_ref().map(Vec::len)
                        .filter(|&len| len > 1)
                        .ok_or(anyhow!(MediatorError::Unavailable("Not enough players in lobby, at least 2 required".to_string())))?;

        let mut deck = self.card_manager.new_deck(1);

        let mut players = self.repo.get_players(game_id).await?;

        self.dealer.initial_deal(&mut deck, &mut players);

        game.inner.state = GameState::Started;
        game.inner.deck = deck;
        
        self.repo.start_game(game, players)
                .await?
                .ok_or(anyhow!(MediatorError::Internal("Couldn't start the game".to_string())))
    }
}
