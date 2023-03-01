use std::sync::Arc;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    domain::entity::user_entity::User,
    infrastructure::response::ApiResponse,
    usecase::{
        create_user_usecase::{CreateUserUsecase, Response},
        find_user_usecase::FindUserUsecase,
        get_all_users_usecase::GetAllUsersUsecase,
        remove_user_usecase::RemoveResponse,
        update_user_usecase::UpdateUserUsecase,
    },
    Repositories,
};

#[get("/", format = "application/json")]
pub fn get_all_users_ctrl(repositories: &State<Arc<Repositories>>) -> ApiResponse<Vec<Response>> {
    let get_all_users_usecase = GetAllUsersUsecase::new(&repositories.user_repo);

    let users = get_all_users_usecase.execute().unwrap();

    let response = users
        .into_iter()
        .map(|user| Response {
            id: user.id,
            name: user.name,
            email: user.email,
        })
        .collect();

    ApiResponse::new(Status::Ok, response, None)
}

#[get("/<id>", format = "application/json")]
pub fn get_user_ctrl(id: String, repositories: &State<Arc<Repositories>>) -> ApiResponse<Response> {
    let find_user_usecase = FindUserUsecase::new(&repositories.user_repo);

    let user = find_user_usecase.execute(id).unwrap();

    ApiResponse::new(
        Status::Ok,
        Response {
            id: user.id,
            name: user.name,
            email: user.email,
        },
        None,
    )
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn create_user_ctrl(
    new_user: Json<NewUser>,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Response> {
    let create_user_usecase = CreateUserUsecase::new(&repositories.user_repo);

    let user = create_user_usecase
        .execute(User::new(new_user.name.clone(), new_user.email.clone()))
        .unwrap();

    ApiResponse::new(
        Status::Created,
        Response {
            id: user.id,
            name: user.name.clone(),
            email: user.email,
        },
        Some(format!("User {} created successfully", user.name)),
    )
}

#[derive(serde::Deserialize)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[patch("/<id>", format = "application/json", data = "<update_user>")]
pub fn update_user_ctrl(
    id: String,
    update_user: Json<UpdateUser>,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Response> {
    let update_user_usecase = UpdateUserUsecase::new(&repositories.user_repo);

    let user = update_user_usecase
        .execute(
            id,
            crate::usecase::update_user_usecase::UpdateUser {
                name: update_user.name.clone(),
                email: update_user.email.clone(),
            },
        )
        .unwrap();

    ApiResponse::new(
        Status::Ok,
        Response {
            id: user.id,
            name: user.name.clone(),
            email: user.email,
        },
        Some(format!("User {} updated successfully", user.name)),
    )
}

#[delete("/<id>", format = "application/json")]
pub fn remove_user_ctrl(
    id: String,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<RemoveResponse> {
    let delete_user_usecase =
        crate::usecase::remove_user_usecase::RemoveUserUsecase::new(&repositories.user_repo);

    delete_user_usecase.execute(id.clone()).unwrap();

    ApiResponse::new(
        Status::Ok,
        RemoveResponse { id: id.clone() },
        Some(format!("User {} removed successfully", id)),
    )
}
