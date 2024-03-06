use rocket::local::asynchronous::Client;
use url_shortener::{diesel_adapter, shortener};

async fn rocket_test() -> Client {
    let rocket = rocket::build()
        .mount("/", shortener::build_routes())
        .attach(diesel_adapter::stage());

    Client::tracked(rocket)
        .await
        .expect("valid rocket instance")
}

#[rocket::async_test]
async fn create_short_url_test() {
    let short_url = "gogle";
    let long_url = "https://www.google.com";

    let client = rocket_test().await;

    let response = client
        .post(format!("/{}", short_url))
        .header(rocket::http::ContentType::JSON)
        .body(format!("{{\"long_url\": \"{}\"}}", long_url))
        .dispatch()
        .await;

    assert_eq!(response.status(), rocket::http::Status::Ok);
}

#[rocket::async_test]
async fn get_url_test() {
    let short_url = "yt";
    let long_url = "https://www.youtube.com";

    let client = rocket_test().await;

    let response = client
        .post(format!("/{}", short_url))
        .header(rocket::http::ContentType::JSON)
        .body(format!("{{\"long_url\": \"{}\"}}", long_url))
        .dispatch()
        .await;

    assert_eq!(response.status(), rocket::http::Status::Ok);

    let response = client.get(format!("/{}", short_url)).dispatch().await;

    assert_eq!(response.status(), rocket::http::Status::SeeOther);
}
