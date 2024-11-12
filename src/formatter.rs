pub enum FormatType {
    Date(String),
    Float(String),
}

pub trait Formatter {
    fn date(&self) -> FormatType;
    fn float(&self) -> FormatType;
}

pub struct CommaFloatWithFrDateFormat;

impl Formatter for CommaFloatWithFrDateFormat {
    fn date(&self) -> FormatType {
        FormatType::Date("%d/%m/%Y".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(",".to_string())
    }
}


pub struct CommaFloatWithIsoDateFormat;

impl Formatter for CommaFloatWithIsoDateFormat {
    fn date(&self) -> FormatType {
        FormatType::Date("%Y-%m-%d".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(",".to_string())
    }
}

pub struct DotFloatWithIsoDateFormat;

impl Formatter for DotFloatWithIsoDateFormat {
    fn date(&self) -> FormatType {
        FormatType::Date("%Y-%m-%d".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(".".to_string())
    }
}
