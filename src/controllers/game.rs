use crate::shithead::game_server::{Game};
use crate::shithead::{CreateGameRequest, CreateGameResponse};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use surrealdb::sql::Thing;
use tonic::{Request, Response, Status};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize)]
struct Partida {
    players: Vec<String>
}

#[derive(Debug)]
pub struct GameService {
    db: Surreal<Db>
}

impl GameService {
    pub fn new(db: Surreal<Db>) -> Self {
        GameService { db }
    }
}

#[tonic::async_trait]
impl Game for GameService {
    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameResponse>, Status > {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let res: Result<Record, surrealdb::Error> = self.db.create("game")
            .content(Partida{
                players: vec![format!("player:{}", req.creator)]
            })
        .await;

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