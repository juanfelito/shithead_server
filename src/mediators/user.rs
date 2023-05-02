use anyhow::Error;
use core::result::Result::Ok;
use crate::models::user::{User};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;

#[derive(Debug)]
pub struct UserMediator {
    repo: SurrealDBRepo
}

impl UserMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        UserMediator { repo }
    }

    pub async fn create_user(&self, name: String) -> Result<WithId<User>,Error> {
        println!("creating a new user...");
        let user: Result<WithId<User>, surrealdb::Error> = self.repo.db.create("user")
            .content(User{
                name
            })
        .await;

        match user {
            Ok(user) => {
                Ok(user)
            }
            Err(_) => {
                Err(Error::msg("couldn't create new user"))
            }
        }
    }
}
