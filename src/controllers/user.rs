use crate::shithead::user_server::{User};
use crate::shithead::{CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse, JoinGameRequest, JoinGameResponse};
use tonic::{Request, Response, Status};
use crate::mediators::player::PlayerMediator;
use crate::mediators::user::UserMediator;

#[derive(Debug)]
pub struct UserService {
    mediator: UserMediator,
    player_mediator: PlayerMediator
}

impl UserService {
    pub fn new(mediator: UserMediator, player_mediator: PlayerMediator) -> Self {
        UserService { mediator, player_mediator }
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn join_game(
        &self,
        request: Request<JoinGameRequest>
    ) -> Result<Response<JoinGameResponse>, Status> {
        println!("got a join game request: {:?}", request);

        let req = request.into_inner();

        let res = self.player_mediator.join_game(req.game_id, req.user_id).await;
        match res {
            Ok(_) => { Ok(Response::new(JoinGameResponse{})) }
            Err(err) => {
                Err(Status::already_exists(format!("couldn't join the requested game: {}", err.to_string())))
            }
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a get request: {:?}", request);

        let req = request.into_inner();

        let res = self.mediator.get_user(req.id).await;
        match res {
            Ok(user) => {
                let reply = GetUserResponse {
                    id: user.id.id.to_string(),
                    name: user.inner.name,
                };
                Ok(Response::new(reply))
            }
            Err(err) => {
                println!("{:?}", err);
                Err(Status::not_found("couldn't find the requested game"))
            }
        }
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status > {
        println!("Got a create user request: {:?}", request);

        let req = request.into_inner();

        let res = self.mediator.create_user(req.name).await;

        match res {
            Ok(created) => {
                let reply = CreateUserResponse {
                    id: created.id.id.to_string()
                };
        
                Ok(Response::new(reply))
            }
            Err(_) => {
                Err(Status::internal("could not create a new user"))
            }
        }
    }
}
