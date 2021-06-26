use serde::{Deserialize, Serialize};

/// Type alias for DB connection pool.
pub type Database = sqlx::PgPool;

/// Creates a new ColorHSV struct.
/// HSV values are stored in the range [0.0, 1.0].
/// Values outside of those bounds will get saturated at construction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "colorHSV")]
pub struct ColorHSV {
    hue: f32,
    sat: f32,
    val: f32,
}

impl ColorHSV {
    /// Initialize a new ColorHSV struct from the given values,
    /// saturating them at the bounds [0.0, 1.0]
    pub fn new(hue: f32, sat: f32, val: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 1.0),
            sat: sat.clamp(0.0, 1.0),
            val: val.clamp(0.0, 1.0),
        }
    }

    /// Returns a tuple containing (hue, sat, val) for
    /// the ColorHSV struct.
    pub fn as_hsv_tuple(&self) -> (f32, f32, f32) {
        (self.hue, self.sat, self.val)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ColorRecord {
    pub name: String,
    pub value: ColorHSV,
}
