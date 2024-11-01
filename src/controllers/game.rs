use crate::shithead::game_server::Game;
use crate::shithead::{CreateGameRequest, CreateGameResponse, GetGameRequest, GetGameResponse, StartGameRequest, StartGameResponse, GetUserResponse};
use tonic::{Request, Response, Status};
use crate::mediators::game::GameMediator;
use crate::mediators::MediatorError;

#[derive(Debug)]
pub struct GameService {
    mediator: GameMediator
}

impl GameService {
    pub fn new(mediator: GameMediator) -> Self {
        GameService { mediator }
    }
}

#[tonic::async_trait]
impl Game for GameService {
    async fn get_game(
        &self,
        request: Request<GetGameRequest>
    ) -> Result<Response<GetGameResponse>, Status> {
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        let res = self.mediator.get_game(req.id).await;
        match res {
            Ok(game) => {
                let users = game.inner.users.unwrap_or_default()
                                    .iter()
                                    .map(|u| {
                                        GetUserResponse{
                                            id: u.id.id.to_string(),
                                            name: u.inner.name.clone(),
                                        }
                                    })
                                    .collect();

                let reply = GetGameResponse {
                    creator_id: game.inner.creator.id.to_string(),
                    deck: game.inner.deck,
                    discard_id: game.inner.discard.id.to_string(),
                    id: game.id.id.to_string(),
                    players_out: game.inner.players_out,
                    state: game.inner.state.into(),
                    turn: game.inner.turn,
                    users: users,
                };
                Ok(Response::new(reply))
            }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                Some(MediatorError::NotFound(_)) => {
                    return Err(Status::not_found(format!("{}: couldn't find the requested game", err)));
                }
                _ => {
                    Err(Status::internal(err.to_string()))
                }
            }
        }
    }

    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameResponse>, Status > {
        println!("Got a create request: {:?}", request);

        let req = request.into_inner();

        let res = self.mediator.create_game(&req.creator).await;

        match res {
            Ok(created) => {
                let reply = CreateGameResponse {
                    id: created.id.id.to_string()
                };
        
                Ok(Response::new(reply))
            }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                _ => {
                    Err(Status::internal(err.to_string()))
                }
            }
        }
    }

    async fn start_game(
        &self,
        request: Request<StartGameRequest>
    ) -> Result<Response<StartGameResponse>, Status> {
        let req = request.into_inner();

        let res = self.mediator.start_game(req.user_id, req.game_id).await;

        match res {
            Ok(_) => {
                Ok(Response::new(StartGameResponse{}))
            }
            Err(err) => match err.downcast_ref::<MediatorError>() {
                Some(err) => {
                    return Err(err.into())
                }
                _ => {
                    Err(Status::internal(format!("Could not start the game: {}", err.to_string())))
                }
            }
        }
    }
}
