use chrono::NaiveDate;

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

    Float(f32),
    FloatOpt(Option<f32>),
    Float64(f64),
    FloatOpt64(Option<f64>),

    Int(i32),
    IntOpt(Option<i32>),
}
