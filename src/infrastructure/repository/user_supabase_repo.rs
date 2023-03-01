use postgrest::Postgrest;
use rocket::serde::json::serde_json::{self, json};

use crate::domain::{
    entity::user_entity::User,
    repository::user_repo::{
        AddError, FindAllError, FindByIdError, RemoveError, UpdateError, UserRepository,
    },
};

pub struct UserSupabaseRepository {
    pub api_url: String,
    pub api_key: String,
    client: Postgrest,
}

impl UserSupabaseRepository {
    pub fn new(api_url: String, api_key: String) -> Self {
        Self {
            api_url: api_url.clone(),
            api_key: api_key.clone(),
            client: Postgrest::new(api_url).insert_header("apikey", api_key),
        }
    }
}

#[async_trait]
impl UserRepository for UserSupabaseRepository {
    async fn find_all_users(&self) -> Result<Vec<User>, FindAllError> {
        match self.client.from("user").select("*").execute().await {
            Ok(response) => {
                let json_text = response.text().await.unwrap();
                let users = serde_json::from_str::<Vec<User>>(json_text.as_str()).unwrap();

                Ok(users)
            }
            Err(_) => Err(FindAllError::Unknown),
        }
    }

    async fn find_user_by_id(&self, id: String) -> Result<User, FindByIdError> {
        match self
            .client
            .from("user")
            .select("*")
            .eq("id", id)
            .execute()
            .await
        {
            Ok(response) => {
                let json_text = response.text().await.unwrap();
                let users = serde_json::from_str::<Vec<User>>(json_text.as_str()).unwrap();

                Ok(users[0].clone())
            }
            Err(_) => Err(FindByIdError::Unknown),
        }
    }

    async fn add_user(&self, user: User) -> Result<User, AddError> {
        let json = json!({
            "name": user.name,
            "email": user.email,
        });

        match self
            .client
            .from("user")
            .insert(json.to_string())
            .execute()
            .await
        {
            Ok(response) => {
                let json_text = response.text().await.unwrap();
                let users = serde_json::from_str::<Vec<User>>(json_text.as_str()).unwrap();

                Ok(users[0].clone())
            }
            Err(_) => Err(AddError::Unknown),
        }
    }

    async fn update_user(&self, user: User) -> Result<User, UpdateError> {
        let json = json!({
            "name": user.name,
            "email": user.email,
        });

        match self
            .client
            .from("user")
            .update(json.to_string())
            .eq("id", user.id)
            .execute()
            .await
        {
            Ok(response) => {
                let json_text = response.text().await.unwrap();
                let users = serde_json::from_str::<Vec<User>>(json_text.as_str()).unwrap();

                Ok(users[0].clone())
            }
            Err(_) => Err(UpdateError::Unknown),
        }
    }

    async fn remove_user(&self, id: String) -> Result<(), RemoveError> {
        match self
            .client
            .from("user")
            .delete()
            .eq("id", id)
            .execute()
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(RemoveError::Unknown),
        }
    }
}
