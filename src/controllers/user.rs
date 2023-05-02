use crate::shithead::user_server::{User};
use crate::shithead::{CreateUserRequest, CreateUserResponse};
use tonic::{Request, Response, Status};
use crate::mediators::user::UserMediator;

#[derive(Debug)]
pub struct UserService {
    mediator: UserMediator
}

impl UserService {
    pub fn new(mediator: UserMediator) -> Self {
        UserService { mediator }
    }
}

#[tonic::async_trait]
impl User for UserService {
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
