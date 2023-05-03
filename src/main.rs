use anyhow::{Result, Ok};
use tonic::{transport::Server};

use shithead::game_server::{GameServer};
use shithead::discard_server::{DiscardServer};
use shithead::user_server::{UserServer};

use controllers::discard::DiscardService;
use controllers::game::GameService;
use controllers::user::UserService;

use mediators::discard::DiscardMediator;
use mediators::game::GameMediator;
use mediators::player::PlayerMediator;
use mediators::user::UserMediator;

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
    let player_mediator = PlayerMediator::new(repo.clone());
    let game_service = GameService::new(game_mediator, player_mediator);

    let discard_mediator = DiscardMediator::new(repo.clone());
    let discard_service = DiscardService::new(discard_mediator);

    let user_mediator = UserMediator::new(repo.clone());
    let user_service = UserService::new(user_mediator);

    Server::builder()
        .add_service(GameServer::new(game_service))
        .add_service(DiscardServer::new(discard_service))
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
