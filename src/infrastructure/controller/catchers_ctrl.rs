use rocket::serde::json::Json;

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
