use rocket::serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ShortLinkRequest<'a> {
    #[validate(length(min = 1, max = 255))]
    pub long_url: &'a str,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ShortLinkResponse {
    pub long_url: String,
    pub short_url: String,
}
