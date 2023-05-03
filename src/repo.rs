use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, File};
use surrealdb::sql::Thing;
use surrealdb::{Error};

use crate::models::WithId;
use crate::models::discard::{Discard, WrappedDiscard};
use crate::models::game::{Game, GameState};
use crate::models::player::Player;
use crate::models::user::User;

#[derive(Clone, Debug)]
pub struct SurrealDBRepo {
    db: Arc<Surreal<Db>>,
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let db = Surreal::new::<File>("shithead.db").await?;
        db.use_ns("shithead").use_db("shithead").await?;
    
        Ok(SurrealDBRepo {
            db: Arc::new(db)
        })
    }

    pub async fn get_game(&self, id: String) -> Result<Option<WithId<Game>>, Error> {
        let sql = format!("select *, <-player<-user as users from game:{}", &id);

        self.db.query(sql).await?.take(0)
    }

    pub async fn get_user(&self, id: String) -> Result<Option<WithId<User>>, Error> {
        self.db.select(("user", id)).await
    }

    pub async fn get_discard(&self, game_id: String) -> Result<Option<WithId<Discard>>, Error> {
        let sql = format!("select discard from game:{} fetch discard", game_id);

        let w_discard: Option<WrappedDiscard> = self.db.query(sql).await?.take(0)?;

        Ok(w_discard.map(|d| d.discard))
    }

    pub async fn create_user(&self, user: User) -> Result<WithId<User>, Error> {
        self.db.create("user").content(user).await
    }

    pub async fn create_game(&self, discard_id: Thing, creator_id: &str) -> Result<WithId<Game>, Error> {
        self.db.create("game")
            .content(Game{
                creator: Thing::from(("user", creator_id)),
                deck: vec![],
                discard: discard_id,
                players_out: vec![],
                state: GameState::Lobby,
                turn: 0,
                users: None,
            })
        .await
    }

    pub async fn create_empty_discard(&self) -> Result<WithId<Discard>, Error> {
        self.db.create("discard").content(Discard::default()).await
    }

    pub async fn join_game(&self, game_id: String, user_id: String, turn: usize) -> Result<Option<WithId<Player>>, Error> {
        let sql = format!("relate user:{}->player->game:{} content {{turn: {}, cards: {{hand: [], face_up: [], face_down: []}}}}", user_id, game_id, turn);

        self.db.query(sql).await?.take(0)
    }
}