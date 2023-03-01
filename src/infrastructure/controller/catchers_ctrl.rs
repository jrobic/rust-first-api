use rocket::{http::Status, serde::json::Json, Request};

use crate::infrastructure::response::ApiResponse;

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[catch(404)]
pub fn not_found_ctrl() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: String::from("Not Found"),
    })
}

#[catch(default)]
pub fn catch_default_ctrl(status: Status, _req: &Request<'_>) -> ApiResponse<Option<()>> {
    let message = Some(status.reason().unwrap_or("An error occured").to_string());

    ApiResponse::new(status, None, message)
}
