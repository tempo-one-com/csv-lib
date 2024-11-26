use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
pub enum QuoteMode {
    None,
    Mixed,
    All,
}

pub enum Eol {
    Unix,
    Windows,
}

pub enum FieldSeparator {
    Comma,
    SemiColumn,
}

#[derive(Debug, Clone)]
pub enum CellType {
    String(String),
    StringOpt(Option<String>),

    Date(NaiveDate),
    DateOpt(Option<NaiveDate>),

    Datetime(NaiveDateTime),
    DatetimeOpt(Option<NaiveDateTime>),

    Float(f32),
    FloatOpt(Option<f32>),
    Float64(f64),
    FloatOpt64(Option<f64>),

    Int(i32),
    IntOpt(Option<i32>),
}

impl From<String> for CellType {
    fn from(value: String) -> Self {
        CellType::String(value)
    }
}

impl From<Option<String>> for CellType {
    fn from(value: Option<String>) -> Self {
        CellType::StringOpt(value)
    }
}

impl From<&str> for CellType {
    fn from(value: &str) -> Self {
        CellType::String(value.to_string())
    }
}

impl From<Option<&str>> for CellType {
    fn from(value: Option<&str>) -> Self {
        CellType::StringOpt(value.map(str::to_string))
    }
}

impl From<NaiveDate> for CellType {
    fn from(value: NaiveDate) -> Self {
        CellType::Date(value)
    }
}

impl From<Option<NaiveDate>> for CellType {
    fn from(value: Option<NaiveDate>) -> Self {
        CellType::DateOpt(value)
    }
}

impl From<NaiveDateTime> for CellType {
    fn from(value: NaiveDateTime) -> Self {
        CellType::Datetime(value)
    }
}

impl From<Option<NaiveDateTime>> for CellType {
    fn from(value: Option<NaiveDateTime>) -> Self {
        CellType::DatetimeOpt(value)
    }
}

impl From<f32> for CellType {
    fn from(value: f32) -> Self {
        CellType::Float(value)
    }
}

impl From<Option<f32>> for CellType {
    fn from(value: Option<f32>) -> Self {
        CellType::FloatOpt(value)
    }
}

impl From<f64> for CellType {
    fn from(value: f64) -> Self {
        CellType::Float64(value)
    }
}

impl From<Option<f64>> for CellType {
    fn from(value: Option<f64>) -> Self {
        CellType::FloatOpt64(value)
    }
}

impl From<i32> for CellType {
    fn from(value: i32) -> Self {
        CellType::Int(value)
    }
}

impl From<Option<i32>> for CellType {
    fn from(value: Option<i32>) -> Self {
        CellType::IntOpt(value)
    }
}
