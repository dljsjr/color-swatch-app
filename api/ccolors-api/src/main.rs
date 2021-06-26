#[macro_use]
extern crate rocket;

use anyhow::{Context, Result};
use rocket::{Build, Rocket};

fn rocket() -> Rocket<Build> {
    rocket::build().attach(postgres::stage_database())
}

#[rocket::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    #[cfg(debug_assertions)]
    {
        dotenv::from_filename("dev.env").context("Couldn't load .env file")?;
    }
    #[cfg(not(debug_assertions))]
    {
        dotenv::dotenv().context("Couldn't load .env file")?;
    }

    env_logger::init();

    match rocket().launch().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub mod postgres;
pub mod types;
