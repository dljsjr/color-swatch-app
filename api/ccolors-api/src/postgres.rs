pub mod routes;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
pub use routes::*;
use sqlx::migrate;
use std::env;

use crate::types::Database;

async fn init_postgres(rocket: Rocket<Build>) -> fairing::Result {
    let db_url = env::var("DATABASE_URL").unwrap_or_default();

    let db = match Database::connect(&db_url).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Couldn't open connection to Postgres Database: {:?}", e);
            return Err(rocket);
        }
    };

    if let Err(e) = migrate!("./migrations").run(&db).await {
        error!("Failed to run init migration for Postgres database: {}", e);
        return Err(rocket);
    }

    Ok(rocket.manage(db))
}

pub fn stage_database() -> AdHoc {
    AdHoc::on_ignite("SQLx Postgres Stage", |rocket| async {
        rocket
            .attach(AdHoc::try_on_ignite(
                "SQLx Postgres Database",
                init_postgres,
            ))
            .mount("/colors", routes![routes::get_colors, routes::add_color])
    })
}
