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

    pub async fn execute(&self, user: User) -> Result<Response, UserException> {
        let user = User::new(user.name, user.email);

        match self.user_repo.add_user(user).await {
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rocket::tokio;

    use crate::{
        domain::{entity::user_entity::User, repository::user_repo::UserRepository},
        infrastructure::repository::user_inmemory_repo::UserInMemoryRepository,
        usecase::create_user_usecase::CreateUserUsecase,
    };

    #[tokio::test]
    async fn create_new_user() {
        let user_in_memory_repo = UserInMemoryRepository::new();
        let user_repo: Arc<dyn UserRepository> = Arc::new(user_in_memory_repo);
        let create_user_usecase = CreateUserUsecase::new(&user_repo);

        let user = create_user_usecase
            .execute(User::new("John".to_string(), "Doe".to_string()))
            .await;

        if user.is_ok() {
            let user = user.unwrap();

            assert_eq!(user.name, "John");
            assert_eq!(user.email, "Doe");

            assert!(!user.id.is_empty());
        }
    }
}
