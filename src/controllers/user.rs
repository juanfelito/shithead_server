use crate::shithead::user_server::User;
use crate::shithead::{CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse};
use tonic::{Request, Response, Status};
use crate::mediators::MediatorError;
use crate::mediators::user::UserMediator;

#[derive(Debug)]
pub struct UserService {
    mediator: UserMediator,
}

impl UserService {
    pub fn new(mediator: UserMediator) -> Self {
        UserService { mediator }
    }
}

#[tonic::async_trait]
impl User for UserService {
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
            Err(err) => {
                Err(Status::internal(format!("Could not create a new user: {}", err.to_string())))
            }
        }
    }
}
