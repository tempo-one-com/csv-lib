pub enum FormatType {
    Date(String),
    Float(String),
}

pub trait Formatter {
    fn date(&self) -> FormatType;
    fn float(&self) -> FormatType;
}

pub struct FormatFr;

impl Formatter for FormatFr {
    fn date(&self) -> FormatType {
        FormatType::Date("%d/%m/%Y".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(",".to_string())
    }
}

pub struct FormatStandard;

impl Formatter for FormatStandard {
    fn date(&self) -> FormatType {
        FormatType::Date("%Y-%m-%d".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(".".to_string())
    }
}
