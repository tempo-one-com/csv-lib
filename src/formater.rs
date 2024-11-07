pub enum FormatType {
    Date(String),
    Float(String),
}

pub trait Formater {
    fn date(&self) -> FormatType;
    fn float(&self) -> FormatType;
}

pub struct FormatFr;

impl Formater for FormatFr {
    fn date(&self) -> FormatType {
        FormatType::Date("%d/%m/%Y".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(",".to_string())
    }
}

pub struct FormatEn;

impl Formater for FormatEn {
    fn date(&self) -> FormatType {
        FormatType::Date("%Y-%m-%d".to_string())
    }

    fn float(&self) -> FormatType {
        FormatType::Float(".".to_string())
    }
}
