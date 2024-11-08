use chrono::NaiveDate;

use crate::formater::{FormatType, Formater};

pub struct Csv<F>
where
    F: Formater + Sized,
{
    pub config: Config,
    pub formater: F,
}

impl<F> Csv<F>
where
    F: Formater + Sized,
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

pub trait CsvStruct {
    fn get_cells(&self) -> Vec<Cell>;
}

pub struct Cell {
    pub title: Option<String>,
    pub value: CellType,
}

impl Cell {
    pub fn new_title(title: &str, value: CellType) -> Self {
        Self {
            title: Some(title.to_string()),
            value,
        }
    }
}

pub struct Config {
    pub eol: Eol,
    pub separator: FieldSeparator,
    pub mode: QuoteMode,
    pub has_header: bool,
}

impl Config {
    pub fn new_unix_fr() -> Self {
        Self {
            eol: Eol::Unix,
            separator: FieldSeparator::SemiColumn,
            mode: QuoteMode::Mixed,
            has_header: true,
        }
    }

    pub fn new_unix_en() -> Self {
        Self {
            eol: Eol::Unix,
            separator: FieldSeparator::Comma,
            mode: QuoteMode::Mixed,
            has_header: true,
        }
    }
}
