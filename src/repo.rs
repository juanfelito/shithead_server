use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, File};
use surrealdb::{Error};

#[derive(Clone, Debug)]
pub struct SurrealDBRepo {
    pub db: Arc<Surreal<Db>>,
}

impl SurrealDBRepo {
    pub async fn init() -> Result<Self, Error> {
        let db = Surreal::new::<File>("shithead.db").await?;
        db.use_ns("shithead").use_db("shithead").await?;
    
        Ok(SurrealDBRepo {
            db: Arc::new(db)
        })
    }
}