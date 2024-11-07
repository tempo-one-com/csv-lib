use chrono::NaiveDate;

mod csv;
mod formater;
mod lang;

#[derive(Debug, Default)]
struct Test {
    name: String,
    date: NaiveDate,
    removed_on: Option<NaiveDate>,
    size: f32,
}

#[cfg(test)]
mod tests {

    use csv::*;
    use formater::{FormatEn, FormatFr};

    use super::*;

    #[test]
    fn fr_linux() {
        let values = vec![
            Test {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: Some(NaiveDate::from_ymd_opt(2024, 11, 07).unwrap()),
                size: 1.79,
            },
            Test {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let csv_cols = vec![
            Cell {
                title: Some("Name".to_string()),
                getter: Box::new(|x: &Test| CellType::String(x.name.clone())),
            },
            Cell {
                title: Some("Taille".to_string()),
                getter: Box::new(|x: &Test| CellType::Float(x.size)),
            },
            Cell {
                title: Some("DOB".to_string()),
                getter: Box::new(|x: &Test| CellType::Date(x.date)),
            },
        ];

        let config = Config {
            eol: Eol::Unix,
            separator: FieldSeparator::SemiColumn,
            mode: QuoteMode::None,
        };
        let formater = FormatFr;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values, csv_cols);
        let expected = r#"Name;Taille;DOB
A;1,790;28/01/2020
B;1,750;11/10/1950"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn en_linux() {
        let values = vec![
            Test {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: None,
                size: 1.79,
            },
            Test {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let csv_cols = vec![
            Cell {
                title: Some("Name".to_string()),
                getter: Box::new(|x: &Test| CellType::String(x.name.clone())),
            },
            Cell {
                title: Some("Taille".to_string()),
                getter: Box::new(|x: &Test| CellType::Float(x.size)),
            },
            Cell {
                title: Some("DOB".to_string()),
                getter: Box::new(|x: &Test| CellType::Date(x.date)),
            },
        ];

        let config = Config {
            eol: Eol::Unix,
            separator: FieldSeparator::Comma,
            mode: QuoteMode::None,
        };
        let formater = FormatEn;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values, csv_cols);
        let expected = r#"Name,Taille,DOB
A,1.790,2020-01-28
B,1.750,1950-10-11"#;

        assert_eq!(result, expected);
    }
}
