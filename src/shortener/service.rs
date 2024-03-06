use rocket::{serde::json::Json, State};

use diesel::prelude::*;

use super::dtos::ShortLinkResponse;
use crate::{
    diesel_adapter::Db,
    error::{Error, Response},
    models::{NewUrl, Url},
    schema::urls,
};

pub async fn create_short_url(
    db: &State<Db>,
    shorten_url: &str,
    long_url: &str,
) -> Response<ShortLinkResponse> {
    let new_url = NewUrl {
        short_url: shorten_url.to_string(),
        long_url: long_url.to_string(),
    };

    match db
        .run(move |conn| {
            diesel::insert_into(urls::table)
                .values(&new_url)
                .execute(conn)
        })
        .await
    {
        Ok(_) => Response::Success(Json(ShortLinkResponse {
            long_url: long_url.to_string(),
            short_url: shorten_url.to_string(),
        })),
        Err(e) => {
            let error = Error {
                message: e.to_string(),
            };

            Response::InternalServerError(Json(error))
        }
    }
}

pub async fn get_url(db: &State<Db>, shorten_url: String) -> Result<String, Response<()>> {
    match db
        .run(move |conn| {
            urls::table
                .filter(urls::short_url.eq(shorten_url))
                .first::<Url>(conn)
        })
        .await
    {
        Ok(url) => Ok(url.long_url),
        Err(_) => Err(Response::NotFound(Json(Error {
            message: "URL not found".to_string(),
        }))),
    }
}
