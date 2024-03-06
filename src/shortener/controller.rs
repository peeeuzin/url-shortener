use super::{
    dtos::{ShortLinkRequest, ShortLinkResponse},
    service,
};
use crate::{
    diesel_adapter::Db,
    error::{Error, Response},
};
use rocket::{response::Redirect, serde::json::Json, Either, State};
use validator::Validate;

#[post("/<shorten_url>", data = "<body>")]
pub async fn create_short_url<'a>(
    db: &'a State<Db>,
    shorten_url: &'a str,
    body: Json<ShortLinkRequest<'a>>,
) -> Response<ShortLinkResponse> {
    let body = body.into_inner();

    if let Err(e) = body.validate() {
        return Response::BadRequest(Json(Error {
            message: e.to_string(),
        }));
    }

    service::create_short_url(db, shorten_url, body.long_url).await
}

#[get("/<shorten_url>")]
pub async fn get_url(db: &State<Db>, shorten_url: &str) -> Either<Redirect, Response<()>> {
    match service::get_url(db, shorten_url.to_string()).await {
        Ok(url) => Either::Left(Redirect::to(url)),
        Err(error) => Either::Right(error),
    }
}
