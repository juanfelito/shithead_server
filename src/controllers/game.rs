use crate::shithead::game_server::{Game};
use crate::shithead::{CreateGameRequest, CreateGameResponse, GetGameRequest, GetGameResponse};
use serde::{Deserialize};
use surrealdb::sql::{Thing};
use tonic::{Request, Response, Status};
use anyhow::{Result, Ok as AnyOk, Error};
use crate::repo::SurrealDBRepo;
use crate::models::{Game as GameModel, WithId};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug)]
pub struct GameService {
    repo: SurrealDBRepo
}

impl GameService {
    pub fn new(repo: SurrealDBRepo) -> Self {
        GameService { repo }
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

        let res = get_game_db(&self.repo, req.id).await;
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

        let res: Result<Record, surrealdb::Error> = self.repo.db.create("game")
            .content(GameModel{
                players: vec![req.creator]
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

async fn get_game_db(repo: &SurrealDBRepo, id: String) -> Result<WithId<GameModel>, Error> {
    println!("trying to get game by id");
    let game: Option<WithId<GameModel>> = repo.db.select(("game", id)).await?;
    match game {
        Some(game) => {
            AnyOk(game)
        }
        None => {
            Err(Error::msg("not found"))
        }
    }
}
