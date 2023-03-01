use crate::domain::entity::user_entity::User;

#[derive(Debug)]
pub enum AddError {
    Conflict,
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

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all_users(&self) -> Result<Vec<User>, FindAllError>;
    async fn find_user_by_id(&self, id: String) -> Result<User, FindByIdError>;
    async fn add_user(&self, user: User) -> Result<User, AddError>;
    async fn update_user(&self, user: User) -> Result<User, UpdateError>;
    async fn remove_user(&self, id: String) -> Result<(), RemoveError>;
}
