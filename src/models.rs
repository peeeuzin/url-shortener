use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub long_url: String,
    pub short_url: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUrl {
    pub long_url: String,
    pub short_url: String,
}
