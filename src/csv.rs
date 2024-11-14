use chrono::NaiveDate;
use toml::Table;

use crate::{
    cell::Cell,
    config::Config,
    formatter::{CellFormatter, FormatType},
    models::{CellType, Eol, FieldSeparator, QuoteMode},
};

/// implémenter ce trait pour sérialiser la struct en question
pub trait CellsBuilder {
    fn get_cells(&self) -> Vec<Cell>;
}

pub struct Csv {
    pub config: Config,
    pub formatter: CellFormatter,
}

impl Csv {
    pub fn new_from_lang(lang: &str) -> Self {
        match lang.to_lowercase().as_str() {
            "fr" => Csv::new_fr(),
            _ => Csv::new_iso(),
        }
    }

    pub fn new_iso() -> Self {
        Self {
            config: Config::new_unix_comma(),
            formatter: CellFormatter::new_iso(),
        }
    }

    pub fn new_fr() -> Self {
        Self {
            config: Config::new_unix_semi_column(),
            formatter: CellFormatter::new_fr(),
        }
    }

    pub fn with_config(self, config: Config) -> Self {
        Self { config, ..self }
    }

    pub fn with_formatter(self, formatter: CellFormatter) -> Self {
        Self { formatter, ..self }
    }

    /// utilise une table de tradution pour les en-tête (passées sous forme de clé dans CsvStruct::get_cells)
    pub fn serialize_i8n_toml<S>(&self, values: &[S], translations: Table) -> String
    where
        S: CellsBuilder,
    {
        self.serialize_opt::<S>(values, Some(translations))
    }

    /// utilise une table de tradution pour les en-tête (passées sous forme de clé dans CsvStruct::get_cells)
    pub fn serialize<S>(&self, values: &[S]) -> String
    where
        S: CellsBuilder,
    {
        self.serialize_opt::<S>(values, None)
    }

    fn serialize_opt<S>(&self, values: &[S], translations: Option<Table>) -> String
    where
        S: CellsBuilder,
    {
        let mut data = String::new();
        let mut header = vec![];
        let mut rows = vec![];

        let quote_mode = self.config.mode.clone();
        let field_separator = match self.config.separator {
            FieldSeparator::Comma => ",",
            FieldSeparator::SemiColumn => ";",
        };

        let eol = match self.config.eol {
            Eol::Unix => "\n",
            Eol::Windows => "\r\n",
        };

        for (i, item) in values.iter().enumerate() {
            let mut row = vec![];

            for cell in item.get_cells() {
                if i == 0 && self.config.has_header {
                    if let Some(title) = cell.title.clone() {
                        let title = match translations.clone() {
                            Some(i18n) => self.get_i18n(&title, &i18n),
                            _ => title,
                        };

                        let value = match quote_mode.clone() {
                            QuoteMode::None => title.to_string(),
                            _ => format!(r#""{title}""#),
                        };

                        header.push(value);
                    }
                }

                let string_value = match cell.value.clone() {
                    CellType::Date(v) | CellType::DateOpt(Some(v)) => self.format_date(v),
                    CellType::Float(v) | CellType::FloatOpt(Some(v)) => self.format_float(v),
                    CellType::Float64(v) | CellType::FloatOpt64(Some(v)) => self.format_float64(v),
                    CellType::Int(v) | CellType::IntOpt(Some(v)) => v.to_string(),
                    CellType::String(v) | CellType::StringOpt(Some(v)) => v.to_string(),
                    _ => String::new(),
                };

                let value = self.quote_value(&string_value, cell, quote_mode.clone());

                row.push(value);
            }

            let row = row.join(field_separator);
            rows.push(row);
        }

        if !header.is_empty() {
            let header = header.join(field_separator);
            data.push_str(&header);
            data.push_str(eol);
        }

        let rows = rows.join(eol);
        data.push_str(&rows);

        data
    }

    fn format_date(&self, value: NaiveDate) -> String {
        match self.formatter.date_format.clone() {
            FormatType::Date(f) => value.format(&f).to_string(),
            _ => unreachable!(),
        }
    }

    fn format_float(&self, value: f32) -> String {
        match self.formatter.float_format.clone() {
            FormatType::Float(f) => format!("{:.3}", value).replace(".", &f),
            _ => unreachable!(),
        }
    }

    fn format_float64(&self, value: f64) -> String {
        match self.formatter.float_format.clone() {
            FormatType::Float(f) => format!("{:.3}", value).replace(".", &f),
            _ => unreachable!(),
        }
    }

    fn quote_value(&self, value: &str, cell: Cell, quote_mode: QuoteMode) -> String {
        match &cell.value {
            CellType::String(_) | CellType::StringOpt(Some(_)) => match quote_mode {
                QuoteMode::None => value.to_string(),
                _ => format!(r#""{value}""#),
            },
            _ => match quote_mode {
                QuoteMode::All => format!(r#""{value}""#),
                _ => value.to_string(),
            },
        }
    }

    fn get_i18n(&self, title: &str, translations: &Table) -> String {
        match title.split(".").collect::<Vec<_>>().as_slice() {
            [section, key] => translations
                .get(*section)
                .and_then(|x| x.get(*key))
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .unwrap_or(title.to_string()),
            [key] => translations
                .get(*key)
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .unwrap_or(title.to_string()),
            _ => title.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn i18n_section() {
        let toml_data = r#"
        [commons]
        recipient = "destinataire"
    "#;

        let translations: Table = toml::from_str(toml_data).unwrap();
        let formatter =
            CellFormatter::new_iso().with_date_format(FormatType::Date("%d/%m%/%Y".to_string()));
        let csv = Csv::new_iso().with_formatter(formatter);
        let result = csv.get_i18n("commons.recipient", &translations);

        assert_eq!("destinataire", result);
    }

    #[test]
    fn i18n_key() {
        let toml_data = r#"
        recipient = "destinataire"
    "#;

        let translations: Table = toml::from_str(toml_data).unwrap();
        let formatter =
            CellFormatter::new_iso().with_date_format(FormatType::Date("%d/%m%/%Y".to_string()));
        let csv = Csv::new_iso().with_formatter(formatter);
        let result = csv.get_i18n("recipient", &translations);

        assert_eq!("destinataire", result);
    }
}
