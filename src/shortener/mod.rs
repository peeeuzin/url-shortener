mod controller;
pub mod dtos;
pub mod service;

pub fn build_routes() -> Vec<rocket::Route> {
    routes![controller::create_short_url, controller::get_url]
}
