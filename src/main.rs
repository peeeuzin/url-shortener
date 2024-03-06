#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Serialize};

use url_shortener::*;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HomeResponse<'a> {
    version: &'a str,
}

#[get("/")]
fn home<'a>() -> Json<HomeResponse<'a>> {
    Json(HomeResponse {
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();

    rocket
        .register("/", catchers![error::catchers])
        .mount("/", shortener::build_routes())
        .mount("/", routes![home])
        .attach(diesel_adapter::stage())
}
