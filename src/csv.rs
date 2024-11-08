use crate::{cell::Cell, config::Config, formatter::{FormatType, Formatter}, models::{CellType, Eol, FieldSeparator, QuoteMode}};

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
    pub fn serialize<S, T>(&self, values: &[S]) -> String
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
                    if let Some(title) = &cell.title {
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
}

