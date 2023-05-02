use anyhow::{Result, Ok};
use tonic::{transport::Server};

use shithead::game_server::{GameServer};
use shithead::discard_server::{DiscardServer};
use controllers::discard::DiscardService;
use controllers::game::GameService;
use mediators::discard::DiscardMediator;
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
    
    let addr = "[::1]:50051".parse()?;
    
    let game_mediator = GameMediator::new(repo.clone());
    let game_service = GameService::new(game_mediator);

    let discard_mediator = DiscardMediator::new(repo.clone());
    let discard_service = DiscardService::new(discard_mediator);

    Server::builder()
        .add_service(GameServer::new(game_service))
        .add_service(DiscardServer::new(discard_service))
        .serve(addr)
        .await?;

    Ok(())
}
