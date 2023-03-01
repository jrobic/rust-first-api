use std::sync::Arc;

use serde::Serialize;

use crate::domain::{
    entity::user_entity::User,
    exception::UserException,
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

impl<'a> CreateUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, user: User) -> Result<Response, UserException> {
        let user = User::new(user.name, user.email);

        match self.user_repo.add_user(user) {
            Ok(user) => Ok(Response {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
            Err(AddError::Conflict) => Err(UserException::Conflict),
            Err(AddError::Unknown) => Err(UserException::Unknown),
        }
    }
}
