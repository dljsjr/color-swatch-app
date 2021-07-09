use crate::types::{ColorFamily, ColorHSV, ColorRecord, Database, RestColorRecord};
use rocket::futures::TryStreamExt;
use rocket::{response::Responder, serde::json::Json, State};
use serde_json::Value;
use sqlx::postgres::PgDatabaseError;

// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// struct PgErrorReponse(String);

// #[derive(Responder)]
// #[response(status = 201, content_type = "json")]
// struct CreatedResponse(String);

#[derive(Responder)]
#[response(content_type = "json")]
pub enum JsonResponse {
    #[response(status = 201)]
    Created(Value),
    #[response(status = 500)]
    DatabaseError(Value),
    #[response(status = 200)]
    Success(Value),
}

use JsonResponse::*;

#[get("/get?<id>")]
pub async fn get_color(db: &State<Database>, id: usize) -> JsonResponse {
    let color = sqlx::query_as!(
        ColorRecord,
        r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE id = $1"#,
        id as i32
    )
    .fetch_one(&**db)
    .await;

    match color {
        Ok(record) => Success(serde_json::to_value(&record).unwrap()),
        Err(e) => DatabaseError(serde_json::json!({
            "error": {
                "message": format!("{:?}", &e)
            }
        })),
    }
}

#[get("/info")]
pub async fn get_info(db: &State<Database>) -> JsonResponse {
    match sqlx::query!("SELECT COUNT(*) FROM colors")
        .fetch_one(&**db)
        .await
    {
        Ok(record) => Success(serde_json::json!({"count": record.count})),
        Err(e) => DatabaseError(serde_json::json!({
            "error": {
                "message": format!("{:?}", &e)
            }
        })),
    }
}

#[get("/?<limit>&<start_at>")]
pub async fn get_colors(db: &State<Database>, limit: usize, start_at: usize) -> JsonResponse {
    let start: i32 = start_at as i32;
    let end: i32 = (start_at + limit - 1) as i32;
    let mut rows = sqlx::query_as!(ColorRecord, r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE id BETWEEN $1 AND $2"#, start, end).fetch(&**db);

    let mut ret: Vec<ColorRecord> = Vec::with_capacity(limit);

    loop {
        match rows.try_next().await {
            Ok(Some(color)) => {
                ret.push(color);
            }
            Ok(None) => {
                break;
            }
            Err(e) => {
                let json = serde_json::json!({
                    "error": {
                        "message": format!("{:?}", &e),
                    }
                });
                return DatabaseError(json);
            }
        }
    }

    Success(serde_json::to_value(&ret).unwrap_or_default())
}

#[get("/search?<query>&<name>&<hex>&<color_family>")]
pub async fn search_colors(
    db: &State<Database>,
    query: &str,
    name: Option<&str>,
    hex: Option<&str>,
    color_family: Option<ColorFamily>,
) -> JsonResponse {
    let mut ret: Vec<ColorRecord> = Vec::with_capacity(10);

    if let Some(family) = color_family {
        let (hue_angle_lower_bound, hue_angle_upper_bound) = family.get_hue_bounds();

        let mut rows = match family {
            // Red has wrap around to the high end so we tweak the query
            ColorFamily::Red => sqlx::query_as!(ColorRecord, r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE (((value).hue BETWEEN $1 AND $2) OR (value).hue BETWEEN 0.96 AND 1.0) AND ((value).sat > 0.05) AND ((value).val BETWEEN 0.1 AND 0.9)"#, hue_angle_lower_bound, hue_angle_upper_bound).fetch(&**db),
            // Brown is a weird duck that shares Hue ranges w/ Orange but has specific saturation and lightness
            ColorFamily::Brown => sqlx::query_as!(ColorRecord, r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE ((value).hue BETWEEN $1 AND $2) AND ((value).sat > 0.23) AND ((value).val BETWEEN 0.2 AND 0.75)"#, hue_angle_lower_bound, hue_angle_upper_bound).fetch(&**db),
            ColorFamily::Gray => sqlx::query_as!(ColorRecord, r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE (value).sat <= 0.05"#).fetch(&**db),
            _ => sqlx::query_as!(ColorRecord, r#"SELECT id as "id: u32", name, value as "value: ColorHSV" FROM colors WHERE ((value).hue BETWEEN $1 AND $2) AND ((value).sat > 0.05) AND ((value).val BETWEEN 0.1 AND 0.9)"#, hue_angle_lower_bound, hue_angle_upper_bound).fetch(&**db)
        };

        loop {
            match rows.try_next().await {
                Ok(Some(color)) => {
                    ret.push(color);
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    let json = serde_json::json!({
                        "error": {
                            "message": format!("{:?}", &e),
                        }
                    });
                    return DatabaseError(json);
                }
            }
        }
    }

    Success(serde_json::to_value(&ret).unwrap())
}

#[post("/add", format = "application/json", data = "<color>")]
pub async fn add_color(db: &State<Database>, color: Json<RestColorRecord>) -> JsonResponse {
    let color: ColorRecord = color.0.into();
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
                let db_error = boxed.downcast_ref::<PgDatabaseError>();

                let json = serde_json::json!({
                    "created": false,
                    "reason": {
                        "message": db_error.message(),
                        "detail": db_error.detail().unwrap_or_default()
                    }
                });

                DatabaseError(json)
            }
            _ => {
                let json = serde_json::json!({
                    "created": false,
                    "reason": {
                        "message": format!("{:?}", &e),
                        "detail": ""
                    }
                });
                DatabaseError(json)
            }
        }
    } else {
        let json = serde_json::json!({"created": true});
        Created(json)
    }
}
