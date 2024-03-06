use std::{env, sync::Arc};

use dotenvy::dotenv;
use rocket::{fairing::AdHoc, tokio::sync::Mutex};

use diesel::{Connection, PgConnection};

pub struct Db(Arc<Mutex<PgConnection>>);

impl Db {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url).expect("Failed to connect to database");
        Db(Arc::new(Mutex::new(conn)))
    }

    pub async fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut PgConnection) -> R + Send + 'static,
        R: Send + 'static,
    {
        let mut conn = self.0.lock().await;
        f(&mut conn)
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Postgres Stage", |rocket| async {
        dotenv().ok();

        let db = Db::init();

        rocket.manage(db)
    })
}
