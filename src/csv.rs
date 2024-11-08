use toml::Table;

use crate::{
    cell::Cell,
    config::Config,
    formatter::{FormatType, Formatter},
    models::{CellType, Eol, FieldSeparator, QuoteMode},
};

/// implémenter ce trait pour sérialiser la struct en question
pub trait CsvStruct {
    fn get_cells(&self) -> Vec<Cell>;
}

pub struct Csv<F>
where
    F: Formatter + Sized,
{
    pub config: Config,
    pub formater: F,
}

impl<F> Csv<F>
where
    F: Formatter + Sized,
{
    /// utilise une table de tradution pour les en-tête (passées sous forme de clé dans CsvStruct::get_cells)
    pub fn serialize_i8n<S>(&self, values: &[S], translations: Table) -> String
    where
        S: CsvStruct,
    {
        self.serialize_opt::<S>(values, Some(translations))
    }

    /// utilise une table de tradution pour les en-tête (passées sous forme de clé dans CsvStruct::get_cells)
    pub fn serialize<S>(&self, values: &[S]) -> String
    where
        S: CsvStruct,
    {
        self.serialize_opt::<S>(values, None)
    }

    fn serialize_opt<S>(&self, values: &[S], translations: Option<Table>) -> String
    where
        S: CsvStruct,
    {
        let mut data = String::new();
        let mut header = vec![];
        let mut rows = vec![];

        let mode = self.config.mode.clone();
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
                            Some(i18n) => self.get_i18n_title(&title, &i18n),
                            _ => title,
                        };

                        let value = match mode {
                            QuoteMode::None => title.to_string(),
                            _ => format!(r#""{title}""#),
                        };

                        header.push(value);
                    }
                }

                let value = match &cell.value {
                    CellType::Date(v) | CellType::DateOpt(Some(v)) => match self.formater.date() {
                        FormatType::Date(f) => {
                            let value = v.format(&f).to_string();

                            match mode {
                                QuoteMode::All => format!(r#""{value}""#),
                                _ => value,
                            }
                        }
                        _ => unreachable!(),
                    },
                    CellType::Float(v) | CellType::FloatOpt(Some(v)) => match self.formater.float()
                    {
                        FormatType::Float(f) => {
                            let value = format!("{:.3}", v).replace(".", &f);

                            match mode {
                                QuoteMode::All => format!(r#""{value}""#),
                                _ => value,
                            }
                        }
                        _ => unreachable!(),
                    },
                    CellType::Float64(v) | CellType::FloatOpt64(Some(v)) => {
                        match self.formater.float() {
                            FormatType::Float(f) => {
                                let value = format!("{:.3}", v).replace(".", &f);

                                match mode {
                                    QuoteMode::All => format!(r#""{value}""#),
                                    _ => value,
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    CellType::String(v) | CellType::StringOpt(Some(v)) => match mode {
                        QuoteMode::None => v.to_string(),
                        _ => format!(r#""{v}""#),
                    },
                    _ => String::new(),
                };

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

    fn get_i18n_title(&self, title: &str, translations: &Table) -> String {
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
    use crate::formatter::FormatFr;

    use super::*;

    #[test]
    fn i18n_section() {
        let toml_data = r#"
        [commons]
        recipient = "destinataire"
    "#;

        let translations: Table = toml::from_str(toml_data).unwrap();
        let csv = Csv {
            config: Config::new_unix_comma(),
            formater: FormatFr,
        };

        let result = csv.get_i18n_title("commons.recipient", &translations);

        assert_eq!("destinataire", result);
    }

    #[test]
    fn i18n_key() {
        let toml_data = r#"
        recipient = "destinataire"
    "#;

        let translations: Table = toml::from_str(toml_data).unwrap();
        let csv = Csv {
            config: Config::new_unix_comma(),
            formater: FormatFr,
        };

        let result = csv.get_i18n_title("recipient", &translations);

        assert_eq!("destinataire", result);
    }

}
