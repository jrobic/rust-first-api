use std::sync::Arc;

use serde::Serialize;

use crate::domain::{
    entity::user_entity::User,
    exception::UserException,
    repository::user_repo::{UpdateError, UserRepository},
};

pub struct UpdateUserUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

impl<'a> UpdateUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(
        &self,
        id: String,
        update_user: UpdateUser,
    ) -> Result<Response, UserException> {
        let user = match self.user_repo.find_user_by_id(id).await {
            Ok(user) => user,
            Err(_) => return Err(UserException::NotFound),
        };

        let user_to_update = User {
            id: user.id,
            name: update_user.name.unwrap_or(user.name),
            email: update_user.email.unwrap_or(user.email),
        };

        match self.user_repo.update_user(user_to_update).await {
            Ok(user) => Ok(Response {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
            Err(UpdateError::NotFound) => Err(UserException::NotFound),
            Err(UpdateError::Unknown) => Err(UserException::Unknown),
        }
    }
}
