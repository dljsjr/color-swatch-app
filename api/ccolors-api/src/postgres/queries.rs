use crate::types::{ColorRecord, Database};
use rocket::{
    http::Status,
    response::{content, status},
    serde::json::Json,
    State,
};
use sqlx::postgres::PgDatabaseError;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/")]
pub async fn index_with_color_count(db: &State<Database>) -> Result<String> {
    let rows = sqlx::query!("SELECT COUNT(*) FROM colors")
        .fetch_one(&**db)
        .await?;

    Ok(if let Some(count) = rows.count {
        format!("Hello! There are {} colors in the database.", count)
    } else {
        "Hello, World! Couldn't get a color count from the database!".to_string()
    })
}

#[post("/colors/add", format = "application/json", data = "<color>")]
pub async fn add_color(
    db: &State<Database>,
    color: Json<ColorRecord>,
) -> status::Custom<content::Json<String>> {
    let (hue, sat, val) = color.value.as_hsv_tuple();
    if let Err(e) = sqlx::query!(
        r#"
INSERT INTO colors ( name, value ) VALUES ( $1, (($2)::colorPart,($3)::colorPart,($4)::colorPart)::colorHSV )
        "#,
        color.name,
        hue as f32,
        sat as f32,
        val as f32
    ).execute(&**db)
    .await {
        match &e {
            sqlx::Error::Database(ref boxed) => {
                log::error!("DB Error!");
                let db_error = boxed.downcast_ref::<PgDatabaseError>();
                
                let json = serde_json::json!({
                    "created": false,
                    "reason": {
                        "message": db_error.message(),
                        "detail": db_error.detail().unwrap_or_default()
                    }
                });

                status::Custom(Status::Ok, content::Json(json.to_string()))
            }
            _ => {
                    let json = serde_json::json!({
                        "created": false,
                        "reason": {
                            "message": format!("{:?}", &e),
                            "detail": ""
                        }
                });
                status::Custom(Status::InternalServerError, content::Json(json.to_string()))
            }
        }
    } else {
        let json = serde_json::json!({"created": true});
        status::Custom(Status::Created, content::Json(json.to_string()))
    }
}
