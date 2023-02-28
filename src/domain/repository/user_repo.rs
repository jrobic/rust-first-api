use crate::domain::entity::user_entity::User;

#[derive(Debug)]
pub enum AddError {
    Unknown,
}

#[derive(Debug)]
pub enum FetchAllError {
    Unknown,
}

pub trait UserRepository: Send + Sync {
    fn add_user(&self, user: User) -> Result<User, AddError>;
    fn fetch_all_users(&self) -> Result<Vec<User>, FetchAllError>;
}
