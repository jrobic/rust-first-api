use rocket::{
    http::{ContentType, Status},
    response::{self, Responder, Response},
    serde::json::serde_json::{self, json},
    Request,
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: Status,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(status: Status, data: T, message: Option<String>) -> Self {
        Self {
            status,
            data: Some(data),
            message,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
          "data": self.data,
          "message": self.message,
          "status": self.status.code,
        })
    }
}

#[rocket::async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.to_json().respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}
