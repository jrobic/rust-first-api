use std::sync::Arc;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    domain::{entity::user_entity::User, exception::UserException},
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

#[get("/")]
pub async fn get_all_users_ctrl(
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Option<Vec<Response>>> {
    let get_all_users_usecase = GetAllUsersUsecase::new(&repositories.user_repo);

    match get_all_users_usecase.execute().await {
        Ok(users) => {
            let response = users
                .into_iter()
                .map(|user| Response {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                })
                .collect();

            ApiResponse::new(Status::Ok, Some(response), None)
        }
        Err(_) => ApiResponse::new(Status::InternalServerError, None, None),
    }
}

#[get("/<id>")]
pub async fn get_user_ctrl(
    id: String,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Option<Response>> {
    let find_user_usecase = FindUserUsecase::new(&repositories.user_repo);

    match find_user_usecase.execute(id).await {
        Ok(user) => ApiResponse::new(
            Status::Ok,
            Some(Response {
                id: user.id,
                name: user.name,
                email: user.email,
            }),
            None,
        ),
        Err(UserException::NotFound) => ApiResponse::new(Status::NotFound, None, None),
        Err(_) => ApiResponse::new(Status::InternalServerError, None, None),
    }
}

#[derive(serde::Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
}

#[post("/", data = "<new_user>")]
pub async fn create_user_ctrl(
    new_user: Json<NewUser>,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Option<Response>> {
    let create_user_usecase = CreateUserUsecase::new(&repositories.user_repo);

    match create_user_usecase
        .execute(User::new(new_user.name.clone(), new_user.email.clone()))
        .await
    {
        Ok(user) => ApiResponse::new(
            Status::Created,
            Some(Response {
                id: user.id,
                name: user.name.clone(),
                email: user.email,
            }),
            Some(format!("User {} created successfully", user.name)),
        ),
        Err(UserException::Conflict) => ApiResponse::new(
            Status::Conflict,
            None,
            Some(format!("User {} already exists", new_user.name)),
        ),
        Err(_) => ApiResponse::new(Status::InternalServerError, None, None),
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[patch("/<id>", data = "<update_user>")]
pub async fn update_user_ctrl(
    id: String,
    update_user: Json<UpdateUser>,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Option<Response>> {
    let update_user_usecase = UpdateUserUsecase::new(&repositories.user_repo);

    match update_user_usecase
        .execute(
            id,
            crate::usecase::update_user_usecase::UpdateUser {
                name: update_user.name.clone(),
                email: update_user.email.clone(),
            },
        )
        .await
    {
        Ok(user) => ApiResponse::new(
            Status::Ok,
            Some(Response {
                id: user.id,
                name: user.name.clone(),
                email: user.email,
            }),
            Some(format!("User {} updated successfully", user.name)),
        ),
        Err(UserException::NotFound) => ApiResponse::new(Status::NotFound, None, None),
        Err(_) => ApiResponse::new(Status::InternalServerError, None, None),
    }
}

#[delete("/<id>")]
pub async fn remove_user_ctrl(
    id: String,
    repositories: &State<Arc<Repositories>>,
) -> ApiResponse<Option<RemoveResponse>> {
    let delete_user_usecase =
        crate::usecase::remove_user_usecase::RemoveUserUsecase::new(&repositories.user_repo);

    match delete_user_usecase.execute(id.clone()).await {
        Ok(_) => ApiResponse::new(
            Status::Ok,
            Some(RemoveResponse { id: id.clone() }),
            Some(format!("User {} removed successfully", id)),
        ),
        Err(UserException::NotFound) => ApiResponse::new(Status::NotFound, None, None),
        Err(_) => ApiResponse::new(Status::InternalServerError, None, None),
    }
}
