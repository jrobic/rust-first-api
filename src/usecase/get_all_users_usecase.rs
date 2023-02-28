use std::sync::Arc;

use serde::Serialize;

use crate::domain::repository::user_repo::{FetchAllError, UserRepository};

pub struct GetAllUsersUsecase<'a> {
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

impl<'a> GetAllUsersUsecase<'a> {
    pub fn new(user_repo: &'a Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub fn execute(&self) -> Result<Vec<Response>, Error> {
        match self.user_repo.fetch_all_users() {
            Ok(users) => {
                let response = users
                    .into_iter()
                    .map(|user| Response {
                        id: String::from(user.id),
                        name: String::from(user.name),
                        email: String::from(user.email),
                    })
                    .collect();
                Ok(response)
            }
            Err(FetchAllError::Unknown) => Err(Error::Unknown),
        }
    }
}
