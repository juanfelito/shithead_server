use anyhow::{Result, Ok};
use tonic::{transport::Server};

use shithead::game_server::{GameServer};
use controllers::game::GameService;

mod repo;
mod models;
mod controllers;
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
    let game_service = GameService::new(repo);

    Server::builder()
        .add_service(GameServer::new(game_service))
        .serve(addr)
        .await?;

    Ok(())
}
