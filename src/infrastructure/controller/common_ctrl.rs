use std::collections::HashMap;

use rocket::{http::HeaderMap, outcome::Outcome, request::FromRequest, serde::json::Json, Request};

#[get("/health")]
pub fn health() -> String {
    "OK".to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub struct RequestHeaders<'h>(&'h HeaderMap<'h>);

#[async_trait]
impl<'r> FromRequest<'r> for RequestHeaders<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let headers = request.headers();
        Outcome::Success(RequestHeaders(headers))
    }
}

#[get("/headers")]
pub fn headers(headers: RequestHeaders) -> Json<HashMap<String, String>> {
    let headers_map: HashMap<String, String> = headers
        .0
        .iter()
        .map(|header| (header.name().to_string(), header.value().to_string()))
        .collect();

    Json(headers_map)
}
