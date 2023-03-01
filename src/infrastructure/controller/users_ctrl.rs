use std::sync::Arc;

use rocket::{serde::json::Json, State};

use crate::{
    domain::entity::user_entity::User,
    usecase::{
        create_user_usecase::{CreateUserUsecase, Response},
        find_user_usecase::FindUserUsecase,
        get_all_users_usecase::GetAllUsersUsecase,
    },
    Repositories,
};

#[get("/", format = "application/json")]
pub fn get_all_users_ctrl(repositories: &State<Arc<Repositories>>) -> Json<Vec<Response>> {
    let get_all_users_usecase = GetAllUsersUsecase::new(&repositories.user_repo);

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

#[get("/<id>", format = "application/json")]
pub fn get_user_ctrl(id: String, repositories: &State<Arc<Repositories>>) -> Json<Response> {
    let find_user_usecase = FindUserUsecase::new(&repositories.user_repo);

    let user = find_user_usecase.execute(&id).unwrap();

    Json(Response {
        id: String::from(user.id),
        name: String::from(user.name),
        email: String::from(user.email),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn create_user_ctrl(
    new_user: Json<NewUser>,
    repositories: &State<Arc<Repositories>>,
) -> Json<Response> {
    let create_user_usecase = CreateUserUsecase::new(&repositories.user_repo);

    let user = create_user_usecase
        .execute(User::new(new_user.name.clone(), new_user.email.clone()))
        .unwrap();

    Json(Response {
        id: String::from(user.id),
        name: String::from(user.name),
        email: String::from(user.email),
    })
}
