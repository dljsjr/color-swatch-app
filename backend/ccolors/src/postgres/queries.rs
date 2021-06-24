use crate::types::Database;
use rocket::State;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index_with_color_count(db: &State<Database>) -> Result<String> {
    let rows = sqlx::query("SELECT * FROM colors").fetch_all(&**db).await?;

    Ok(format!(
        "Hello! There are {} colors in the database.",
        rows.len()
    ))
}
