use crate::domain::entity::user_entity::User;

#[derive(Debug)]
pub enum AddError {
    Unknown,
}

#[derive(Debug)]
pub enum FindAllError {
    Unknown,
}

pub enum FindByIdError {
    Unknown,
    NotFound,
}

pub enum UpdateError {
    Unknown,
    NotFound,
}

pub enum RemoveError {
    Unknown,
    NotFound,
}

pub trait UserRepository: Send + Sync {
    fn find_all_users(&self) -> Result<Vec<User>, FindAllError>;
    fn find_user_by_id(&self, id: String) -> Result<User, FindByIdError>;
    fn add_user(&self, user: User) -> Result<User, AddError>;
    fn update_user(&self, user: User) -> Result<User, UpdateError>;
    fn remove_user(&self, id: String) -> Result<(), RemoveError>;
}
