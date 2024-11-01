use anyhow::{anyhow, Error};
use core::result::Result::Ok;
use crate::models::user::{User};
use crate::models::WithId;
use crate::repo::SurrealDBRepo;
use super::MediatorError;

#[derive(Debug)]
pub struct UserMediator {
    repo: SurrealDBRepo
}

impl UserMediator {
    pub fn new(repo: SurrealDBRepo) -> Self {
        UserMediator { repo }
    }

    pub async fn get_user(&self, id: String) -> Result<WithId<User>,Error> {
        self.repo.get_user(id)
            .await?
            .ok_or(anyhow!(MediatorError::NotFound("User not found".to_string())))
    }

    pub async fn create_user(&self, name: String) -> Result<WithId<User>, Error> {
        let user: WithId<User> = self.repo.create_user(User { name }).await?;

        Ok(user)
    }
}
