use crate::shithead::game_server::{Game};
use crate::shithead::{CreateGameRequest, CreateGameResponse, GetGameRequest, GetGameResponse};
use tonic::{Request, Response, Status};
use crate::mediators::game::GameMediator;

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
                let reply = GetGameResponse {
                    id: game.id.id.to_string(),
                    players: game.inner.players,
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

        let res = self.mediator.create_game(req.creator).await;

        match res {
            Ok(created) => {
                let reply = CreateGameResponse {
                    id: created.id.to_string()
                };
        
                Ok(Response::new(reply))
            }
            Err(_) => {
                Err(Status::internal("could not create a new game"))
            }
        }
    }
}
