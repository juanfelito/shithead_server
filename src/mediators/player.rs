use anyhow::{anyhow, Result, Error, Ok};
use crate::dealer::Dealer;
use crate::models::game::{Game, GameState};
use crate::models::player::Player;
use crate::models::WithId;
use crate::repo::SurrealDBRepo;
use super::MediatorError;
use crate::card_manager::{Card, CardValue};

#[derive(Debug)]
pub struct PlayerMediator {
    repo: SurrealDBRepo
}

impl PlayerMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        PlayerMediator { repo }
    }

    pub async fn play(
        &self, player_id: String, chosen_cards: Vec<String>
    ) -> Result<(Vec<String>, String, bool), Error> {
        let (player, game) = self.repo.get_player_and_game(player_id).await?;

        let mut player = player.ok_or(anyhow!(MediatorError::NotFound("Player not found".to_string())))?;
        let mut game = game.ok_or(anyhow!(MediatorError::NotFound("Game not found".to_string())))?;

        match game.inner.state {
            GameState::Lobby => {
                return Err(anyhow!(MediatorError::Unavailable("The game hasn't started yet!".to_string())));
            },
            GameState::Finished => {
                if !Dealer::is_empty(&player.inner) {
                    return Err(anyhow!(MediatorError::Unavailable("You're a S H I T H E A D ! !".to_string())));
                }
                return Err(anyhow!(MediatorError::Unavailable("The game is over!".to_string())));
            },
            _ => {}
        }
        
        if game.inner.turn != player.inner.turn {
            return Err(anyhow!(MediatorError::Unavailable("It's not this player's turn".to_string())));
        }

        let cards = Dealer::get_active_cards(&mut player.inner);

        // check lenght of input against active cards

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

        let play_cards: Vec<Card> = cards_to_check.iter().map(|c| Card::parse_card(c).unwrap()).collect();

        if !Dealer::of_the_same_value(&play_cards) {
            return Err(anyhow!(MediatorError::InvalidArgument("All played cards must have the same value!".to_string())));
        }

        let mut discard = self.repo.get_discard(game.id.id.to_string())
                            .await?.ok_or(anyhow!(MediatorError::Internal("Discard not found for game".to_string())))?;

        let current_value: Option<CardValue> = CardValue::parse(discard.inner.current_value.clone());
        
        let cards_added_to_player: Vec<String>;
        let mut message: String;
        let mut burned = false;

        if !play_cards[0].can_be_played_on(&current_value) {
            cards_added_to_player = Dealer::unsuccessful_play(&mut player.inner, &mut discard.inner, &mut cards_to_check);
            message = format!("Play not allowed: {} can't be played on a {:?}", play_cards[0].value, current_value.expect("you can always play over empty"));
        } else {
            message = "It can be played!".to_string();
            Dealer::successful_play(&mut discard.inner, &mut cards_to_check);

            let current_value = CardValue::parse(discard.inner.current_value.clone());
            if current_value == Some(CardValue::Ten) || discard.inner.repeat_count >= 4 {
                message = "K A A A B O O O O O O S H ! !".to_string();
                Dealer::burn_discard(&mut discard.inner);
                burned = true;
            }

            cards_added_to_player = Dealer::take_from_deck(&mut player.inner, &mut game.inner.deck);
            if Dealer::is_empty(&player.inner) {
                game.inner.players_out.push(player.inner.turn);
                message = "Congratulations, you're not a shithead!".to_string();

                if game.inner.players_out.len() == game.inner.users.as_ref().unwrap().len() - 1 {
                    message = "The game is over, congratulations, you're not a shithead!".to_string();
                    game.inner.state = GameState::Finished;
                }
            }
        }

        if !burned {
            Dealer::tick_turn(&mut game.inner);
        }

        let err = self.repo.commit_play(game, discard, player).await;
        if err.is_some() {
            return Err(anyhow!(MediatorError::Internal("Error saving your play".to_string())));
        }

        Ok((cards_added_to_player, message, burned))
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

    pub async fn get_opponents(&self, game_id: String, player_id: String) -> Result<Vec<WithId<Player>>, Error> {
        let res = self.repo.get_players(game_id).await;
        
        if res.is_err() {
            return Err(anyhow!(MediatorError::NotFound("Players not found".to_string())));
        }

        let mut players = res.unwrap();
        players.retain(|p| p.id.id.to_string() != player_id);

        return Ok(players);
    }
}
