use std::sync::Arc;

use rocket::{serde::json::Json, State};

use crate::{
    domain::repository::user_repo::UserRepository,
    usecase::{create_user_usecase::Response, get_all_users_usecase::GetAllUsersUsecase},
};

#[get("/", format = "application/json")]
pub fn execute(user_repo: &State<Arc<dyn UserRepository>>) -> Json<Vec<Response>> {
    let get_all_users_usecase = GetAllUsersUsecase::new(user_repo);

    let users = get_all_users_usecase.execute().unwrap();

    let response = users
        .into_iter()
        .map(|user| Response {
            id: String::from(user.id),
            name: String::from(user.name),
            email: String::from(user.email),
        })
        .collect();

    Json(response)
}
