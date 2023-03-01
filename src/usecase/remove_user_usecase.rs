use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::{
    exception::UserException,
    repository::user_repo::{RemoveError, UserRepository},
};

pub struct RemoveUserUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveResponse {
    pub id: String,
}

impl<'a> RemoveUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, id: String) -> Result<RemoveResponse, UserException> {
        match self.user_repo.remove_user(id.clone()) {
            Ok(_user) => Ok(RemoveResponse { id }),
            Err(RemoveError::NotFound) => Err(UserException::NotFound),
            Err(RemoveError::Unknown) => Err(UserException::Unknown),
        }
    }
}
