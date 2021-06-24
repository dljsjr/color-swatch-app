#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use rocket::{Build, Rocket};
use sqlx::postgres::PgPool;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}

#[rocket::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().context("Couldn't load .env file")?;

    // Get the DB url from the environment
    let db_url =
        env::var("DATABASE_URL").context("Could not get DATABASE_URL from environment.")?;

    // Create the sqlx connection pool
    let _db_pool = PgPool::connect(&db_url).await.context(format!(
        "Couldn't open connection to database at URL {}",
        &db_url
    ))?;

    match rocket().launch().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
