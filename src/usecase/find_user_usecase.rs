use std::sync::Arc;

use serde::Serialize;

use crate::domain::repository::user_repo::{FindByIdError, UserRepository};

pub struct FindUserUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub enum Error {
    Unknown,
    NotFound,
}

impl<'a> FindUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, user_id: &str) -> Result<Response, Error> {
        match self.user_repo.find_user_by_id(user_id) {
            Ok(user) => Ok(Response {
                id: String::from(user.id),
                name: String::from(user.name),
                email: String::from(user.email),
            }),
            Err(FindByIdError::NotFound) => Err(Error::NotFound),
            Err(FindByIdError::Unknown) => Err(Error::Unknown),
        }
    }
}
