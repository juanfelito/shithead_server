use anyhow::{Result, Ok};
use tonic::{transport::Server};

use shithead::game_server::{GameServer};
use controllers::game::GameService;
use mediators::game::GameMediator;

mod controllers;
mod mediators;
mod models;
mod repo;

pub mod shithead {
    tonic::include_proto!("shithead");
}

#[tokio::main]
async fn main() -> Result<()> {
    let repo = repo::SurrealDBRepo::init().await?;
    // let sql = "SELECT * FROM game";
    // let res = db.query(sql).await?;

    // println!("{:?}", res);
    
    let addr = "[::1]:50051".parse()?;

    let game_mediator = GameMediator::new(repo);
    let game_service = GameService::new(game_mediator);

    Server::builder()
        .add_service(GameServer::new(game_service))
        .serve(addr)
        .await?;

    Ok(())
}
