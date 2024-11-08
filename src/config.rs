use crate::models::{Eol, FieldSeparator, QuoteMode};

pub struct Config {
    pub eol: Eol,
    pub separator: FieldSeparator,
    pub mode: QuoteMode,
    pub has_header: bool,
}

impl Config {
    pub fn new_unix_semi_column() -> Self {
        Self {
            eol: Eol::Unix,
            separator: FieldSeparator::SemiColumn,
            mode: QuoteMode::Mixed,
            has_header: true,
        }
    }

    pub fn new_unix_comma() -> Self {
        Self {
            eol: Eol::Unix,
            separator: FieldSeparator::Comma,
            mode: QuoteMode::Mixed,
            has_header: true,
        }
    }

    pub fn with_eol(self, eol: Eol) -> Self {
        Self {
            eol,
            ..self
        }
    }

    pub fn with_mode(self, mode: QuoteMode) -> Self {
        Self {
            mode,
            ..self
        }
    }

    pub fn with_separator(self, separator: FieldSeparator) -> Self {
        Self {
            separator,
            ..self
        }
    }

    pub fn with_header(self, has_header: bool) -> Self {
        Self {
            has_header,
            ..self
        }
    }    
}