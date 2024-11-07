use chrono::NaiveDate;

use crate::formater::{FormatType, Formater};

type CellFormater<S, T> = Box<dyn Fn(&S) -> T>;

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
    pub fn serialize<S, T>(&self, values: &[S], columns: Vec<Cell<S, CellType>>) -> String {
        let mut data = String::new();
        let mut header = vec![];
        let mut rows = vec![];

        let field_separator = match self.config.separator {
            FieldSeparator::Comma => ",",
            FieldSeparator::SemiColumn => ";",
        };

        let eol = match self.config.eol {
            Eol::Unix => "\n",
            Eol::Windows => "\r\n",
        };

        for (i, item) in values.into_iter().enumerate() {
            let mut row = vec![];

            for col in &columns {
                if i == 0 {
                    if let Some(title) = col.title.clone() {
                        header.push(title);
                    }
                }

                let value = match (col.getter)(item) {
                    CellType::Date(v) => match self.formater.date() {
                        FormatType::Date(f) => v.format(&f).to_string(),
                        _ => unreachable!(),
                    },
                    CellType::Float(v) => match self.formater.float() {
                        FormatType::Float(f) => {
                            let float_str = format!("{:.3}", v);
                            float_str.replace(".", &f)
                        }
                        _ => unreachable!(),
                    },
                    CellType::String(v) => v,
                };

                row.push(value);
            }

            let row = row.join(field_separator);
            rows.push(row);
        }

        let header = header.join(field_separator);

        data.push_str(&header);
        data.push_str(&eol);
        let rows = rows.join(&eol);
        data.push_str(&rows);

        data
    }
}

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
    Date(NaiveDate),
    Float(f32),
}

pub struct Cell<S, T> {
    pub title: Option<String>,
    pub getter: Box<dyn Fn(&S) -> T>,
}

pub struct Config {
    pub eol: Eol,
    pub separator: FieldSeparator,
    pub mode: QuoteMode,
}
