use crate::shithead::player_server::{Player};
use crate::shithead::{JoinGameRequest, JoinGameResponse, GetPlayerRequest, GetPlayerResponse, PlayRequest, PlayResponse};
use tonic::{Request, Response, Status};
use crate::mediators::MediatorError;
use crate::mediators::player::PlayerMediator;

#[derive(Debug)]
pub struct PlayerService {
    mediator: PlayerMediator
}

impl PlayerService {
    pub fn new(mediator: PlayerMediator) -> Self {
        PlayerService { mediator }
    }
}

#[tonic::async_trait]
impl Player for PlayerService {
    async fn play(
        &self,
        request: Request<PlayRequest>
    ) -> Result<Response<PlayResponse>, Status> {
        println!("got a play request: {:?}", request);
        let req = request.into_inner();

        let res = self.mediator.play(req.player_id, req.cards).await;

        match res {
            Ok(_) => { Ok(Response::new(PlayResponse { cards: vec![], turn_ended: false })) }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                Some(err) => {
                    return Err(err.into());
                }
                _ => {
                    Err(Status::internal(format!("Could not play the turn: {}", err.to_string())))
                }
            }
        }
    }

    async fn join_game(
        &self,
        request: Request<JoinGameRequest>
    ) -> Result<Response<JoinGameResponse>, Status> {
        println!("got a join game request: {:?}", request);
        let req = request.into_inner();

        let res = self.mediator.join_game(req.game_id, req.user_id).await;
        match res {
            Ok(_) => { Ok(Response::new(JoinGameResponse{})) }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                Some(err) => {
                    return Err(err.into());
                }
                _ => {
                    Err(Status::internal(format!("Could not join the game: {}", err.to_string())))
                }
            }
        }
    }

    async fn get_player(
        &self,
        request: Request<GetPlayerRequest>
    ) -> Result<Response<GetPlayerResponse>, Status> {
        println!("got a get player request: {:?}", request);
        let req = request.into_inner();

        let res = self.mediator.get_player(req.game_id, req.user_id).await;

        match res {
            Ok(player) => {
                let reply = GetPlayerResponse {
                    id: player.id.id.to_string(),
                    turn: player.inner.turn,
                    cards: player.inner.cards.into(),
                };
                Ok(Response::new(reply))
            }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                Some(err) => {
                    return Err(err.into());
                }
                _ => {
                    Err(Status::internal(err.to_string()))
                }
            }
        }
    }
}
