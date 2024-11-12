#[derive(Clone)]
pub enum FormatType {
    Date(String),
    Float(String),
}

pub struct CellFormatter {
    pub date_format: FormatType,
    pub float_format: FormatType,
}

impl CellFormatter {
    pub fn new_iso() -> Self {
        Self {
            date_format: FormatType::Date("%Y-%m-%d".to_string()),
            float_format: FormatType::Float(".".to_string()),
        }
    }

    pub fn new_fr() -> Self {
        Self {
            date_format: FormatType::Date("%d/%m/%Y".to_string()),
            float_format: FormatType::Float(",".to_string()),
        }
    }

    pub fn with_date_format(self, date_format: FormatType) -> Self {
        Self {
            date_format,
            ..self
        }
    }

    pub fn with_float_format(self, float_format: FormatType) -> Self {
        Self {
            float_format,
            ..self
        }
    }
}
