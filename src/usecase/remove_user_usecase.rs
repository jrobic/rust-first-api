use std::sync::Arc;

use serde::Serialize;

use crate::domain::repository::user_repo::{RemoveError, UserRepository};

pub struct RemoveUserUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

#[derive(Debug, Serialize)]
pub struct RemoveResponse {
    pub id: String,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    NotFound,
}

impl<'a> RemoveUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, id: String) -> Result<RemoveResponse, Error> {
        match self.user_repo.remove_user(id.clone()) {
            Ok(_user) => Ok(RemoveResponse { id }),
            Err(RemoveError::NotFound) => Err(Error::NotFound),
            Err(RemoveError::Unknown) => Err(Error::Unknown),
        }
    }
}
