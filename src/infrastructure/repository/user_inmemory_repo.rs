use std::sync::Mutex;

use crate::domain::{
    entity::user_entity::User,
    repository::user_repo::{AddError, FetchAllError, UserRepository},
};

pub struct UserInMemoryRepository {
    pub users: Mutex<Vec<User>>,
}

impl UserInMemoryRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(vec![]),
        }
    }
}

impl UserRepository for UserInMemoryRepository {
    fn add_user(&self, user: User) -> Result<User, AddError> {
        format!("user added: {:?}", user);

        let mut lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(AddError::Unknown),
        };

        lock.push(user.clone());

        Ok(user)
    }

    fn fetch_all_users(&self) -> Result<Vec<User>, FetchAllError> {
        let lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(FetchAllError::Unknown),
        };

        Ok(lock.to_vec())
    }
}
