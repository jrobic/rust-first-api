use std::sync::Mutex;

use crate::domain::{
    entity::user_entity::User,
    repository::user_repo::{
        AddError, FindAllError, FindByIdError, RemoveError, UpdateError, UserRepository,
    },
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

    fn find_all_users(&self) -> Result<Vec<User>, FindAllError> {
        let lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(FindAllError::Unknown),
        };

        Ok(lock.to_vec())
    }

    fn find_user_by_id(&self, id: &str) -> Result<User, FindByIdError> {
        let lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(FindByIdError::Unknown),
        };

        match lock.iter().find(|user| user.id == id) {
            Some(user) => Ok(user.clone()),
            None => Err(FindByIdError::NotFound),
        }
    }

    fn update_user(&self, user: User) -> Result<User, UpdateError> {
        let mut lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(UpdateError::Unknown),
        };

        match lock.iter_mut().find(|u| u.id == user.id) {
            Some(u) => {
                u.name = user.name;
                u.email = user.email;
                Ok(u.clone())
            }
            None => Err(UpdateError::NotFound),
        }
    }

    fn remove_user(&self, id: &str) -> Result<(), RemoveError> {
        let mut lock = match self.users.lock() {
            Ok(lock) => lock,
            _ => return Err(RemoveError::Unknown),
        };

        match lock.iter().position(|user| user.id == id) {
            Some(index) => {
                lock.remove(index);
                Ok(())
            }
            None => Err(RemoveError::NotFound),
        }
    }
}
