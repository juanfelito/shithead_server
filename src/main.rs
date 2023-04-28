use anyhow::{Result, Ok};
use surrealdb::Surreal;
use surrealdb::engine::local::File;
use tonic::{transport::Server};

use shithead::game_server::{GameServer};
use controllers::game::GameService;

mod controllers;
pub mod shithead {
    tonic::include_proto!("shithead");
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Surreal::new::<File>("temp.db").await?;
    db.use_ns("shithead").use_db("shithead").await?;

    // let sql = "SELECT * FROM game";
    // let res = db.query(sql).await?;

    // println!("{:?}", res);
    
    let addr = "[::1]:50051".parse()?;
    let game_service = GameService::new(db);

    Server::builder()
        .add_service(GameServer::new(game_service))
        .serve(addr)
        .await?;

    Ok(())
}
