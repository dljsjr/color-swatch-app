use serde::{Deserialize, Serialize};

/// Type alias for DB connection pool.
pub type Database = sqlx::PgPool;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
pub struct ColorPart(f32);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Copy, FromFormField)]
#[serde(crate = "rocket::serde")]
pub enum ColorFamily {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Brown,
    Gray,
}

impl ColorFamily {
    /// Hue bounds taken from here: https://en.wikipedia.org/wiki/Hue
    pub fn get_hue_bounds(&self) -> (f32, f32) {
        match self {
            ColorFamily::Red => (0.0f32, 15.0f32 / 360.0f32),
            ColorFamily::Orange => (15.0f32 / 360.0f32, 45.0f32 / 360.0f32),
            ColorFamily::Yellow => (45.0f32 / 360.0f32, 75.0f32 / 360.0f32),
            ColorFamily::Green => (75.0f32 / 360.0f32, 180.0f32 / 360.0f32),
            ColorFamily::Blue => (165.0f32 / 360.0f32, 270.0f32 / 360.0f32),
            ColorFamily::Purple => (255.0f32 / 360.0f32, 359.0f32 / 360.0f32),
            ColorFamily::Brown => ColorFamily::Orange.get_hue_bounds(),
            ColorFamily::Gray => (0.0f32, 1.0f32),
        }
    }
}

/// Creates a new ColorHSV struct.
/// HSV values are stored in the range [0.0, 1.0].
/// Values outside of those bounds will get saturated at construction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "colorHSV")]
pub struct ColorHSV {
    hue: ColorPart,
    sat: ColorPart,
    val: ColorPart,
}

impl ColorHSV {
    /// Initialize a new ColorHSV struct from the given values,
    /// saturating them at the bounds [0.0, 1.0]
    pub fn new(hue: f32, sat: f32, val: f32) -> Self {
        Self {
            hue: ColorPart(hue.clamp(0.0, 1.0)),
            sat: ColorPart(sat.clamp(0.0, 1.0)),
            val: ColorPart(val.clamp(0.0, 1.0)),
        }
    }

    /// Returns a tuple containing (hue, sat, val) for
    /// the ColorHSV struct.
    pub fn as_hsv_tuple(&self) -> (f32, f32, f32) {
        (self.hue.0, self.sat.0, self.val.0)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
pub struct ColorRecord {
    pub id: u32,
    pub name: String,
    pub value: ColorHSV,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
pub struct RestColorRecord {
    pub id: u32,
    pub name: String,
    pub value: RestColorHSV,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::Type)]
#[serde(crate = "rocket::serde")]
#[sqlx(type_name = "colorHSV")]
pub struct RestColorHSV {
    hue: f32,
    sat: f32,
    val: f32,
}

impl From<RestColorRecord> for ColorRecord {
    fn from(other: RestColorRecord) -> Self {
        Self {
            id: other.id,
            name: other.name,
            value: ColorHSV {
                hue: ColorPart(other.value.hue),
                sat: ColorPart(other.value.sat),
                val: ColorPart(other.value.val),
            },
        }
    }
}
