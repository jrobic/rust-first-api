use std::sync::Arc;

use crate::domain::{
    entity::user_entity::User,
    exception::UserException,
    repository::user_repo::{FindAllError, UserRepository},
};

pub struct GetAllUsersUsecase<'a> {
    pub user_repo: &'a Arc<dyn UserRepository>,
}

impl<'a> GetAllUsersUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self) -> Result<Vec<User>, UserException> {
        match self.user_repo.find_all_users().await {
            Ok(users) => {
                let response = users
                    .into_iter()
                    .map(|user| User {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                    })
                    .collect();
                Ok(response)
            }
            Err(FindAllError::Unknown) => Err(UserException::Unknown),
        }
    }
}
