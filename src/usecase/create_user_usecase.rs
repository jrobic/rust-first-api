use std::sync::Arc;

use serde::Serialize;

use crate::domain::{
    entity::user_entity::User,
    repository::user_repo::{AddError, UserRepository},
};

pub struct CreateUserUsecase<'a> {
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
}

impl<'a> CreateUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, user: User) -> Result<Response, Error> {
        let user = User::new(user.name, user.email);

        match self.user_repo.add_user(user) {
            Ok(user) => Ok(Response {
                id: String::from(user.id),
                name: String::from(user.name),
                email: String::from(user.email),
            }),
            Err(AddError::Unknown) => Err(Error::Unknown),
        }
    }
}
