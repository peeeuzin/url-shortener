use rocket::{
    http::Status,
    serde::{json::Json, Serialize},
    Request, Responder,
};
use std::fmt::Debug;

#[derive(Serialize, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    pub message: String,
}

#[derive(Responder, PartialEq, Debug)]
pub enum Response<T: Serialize + PartialEq + Debug> {
    Success(Json<T>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<Error>),
    #[response(status = 500, content_type = "json")]
    InternalServerError(Json<Error>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<Error>),
}

#[catch(default)]
pub fn catchers(status: Status, _: &Request) -> Json<Error> {
    let error = Error {
        message: status.reason().unwrap_or("Unknown error").to_string(),
    };

    Json(error)
}
