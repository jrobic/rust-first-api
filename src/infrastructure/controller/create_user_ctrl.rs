use std::sync::Arc;

use rocket::{serde::json::Json, State};

use crate::{
    domain::{entity::user_entity::User, repository::user_repo::UserRepository},
    usecase::create_user_usecase::{CreateUserUsecase, Response},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn execute(
    new_user: Json<NewUser>,
    user_repo: &State<Arc<dyn UserRepository>>,
) -> Json<Response> {
    let create_user_usecase = CreateUserUsecase::new(user_repo);

    let user = create_user_usecase
        .execute(User::new(new_user.name.clone(), new_user.email.clone()))
        .unwrap();

    // Json(user)
    Json(Response {
        id: String::from(user.id),
        name: String::from(user.name),
        email: String::from(user.email),
    })
}
