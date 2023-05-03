use crate::shithead::game_server::{Game};
use crate::shithead::{CreateGameRequest, CreateGameResponse, GetGameRequest, GetGameResponse};
use tonic::{Request, Response, Status};
use crate::mediators::game::GameMediator;
use crate::mediators::player::PlayerMediator;

#[derive(Debug)]
pub struct GameService {
    mediator: GameMediator,
    player_mediator: PlayerMediator
}

impl GameService {
    pub fn new(mediator: GameMediator, player_mediator: PlayerMediator) -> Self {
        GameService { mediator, player_mediator }
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
                let reply = GetGameResponse {
                    creator: game.inner.creator.id.to_string(),
                    deck: game.inner.deck,
                    discard_id: game.inner.discard.id.to_string(),
                    id: game.id.id.to_string(),
                    players_out: game.inner.players_out,
                    state: game.inner.state.into(),
                    turn: game.inner.turn,
                };
                Ok(Response::new(reply))
            }
            Err(err) => {
                println!("{:?}", err);
                Err(Status::not_found("couldn't find the requested game"))
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
                // TODO save result and add player id to the response
                self.player_mediator.join_game(created.id.id.to_string(), req.creator).await;
                let reply = CreateGameResponse {
                    id: created.id.id.to_string()
                };
        
                Ok(Response::new(reply))
            }
            Err(_) => {
                Err(Status::internal("could not create a new game"))
            }
        }
    }
}
