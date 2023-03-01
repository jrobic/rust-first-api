use std::sync::Arc;

use crate::domain::{
    entity::user_entity::User,
    exception::UserException,
    repository::user_repo::{FindByIdError, UserRepository},
};

pub struct FindUserUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

impl<'a> FindUserUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self, user_id: String) -> Result<User, UserException> {
        match self.user_repo.find_user_by_id(user_id) {
            Ok(user) => Ok(User {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
            Err(FindByIdError::NotFound) => Err(UserException::NotFound),
            Err(FindByIdError::Unknown) => Err(UserException::Unknown),
        }
    }
}
