pub mod cell;
pub mod config;
pub mod csv;
pub mod formatter;
pub mod models;

#[cfg(test)]
mod tests {

    use cell::Cell;
    use chrono::NaiveDate;
    use config::Config;
    use csv::*;
    use models::{CellType, QuoteMode};

    use super::*;

    #[derive(Debug, Default)]
    struct Basic {
        name: String,
        size: f32,
    }

    impl CellsBuilder for Basic {
        fn get_cells(&self) -> Vec<Cell> {
            vec![
                Cell::new_title("Name", CellType::String(self.name.clone())),
                Cell::new_title("Taille", CellType::Float(self.size)),
            ]
        }
    }

    #[derive(Debug, Default)]
    struct Person {
        name: String,
        date: NaiveDate,
        removed_on: Option<NaiveDate>,
        size: f32,
    }

    impl CellsBuilder for Person {
        fn get_cells(&self) -> Vec<Cell> {
            vec![
                Cell::new_title("Name", CellType::String(self.name.clone())),
                Cell::new_title("Taille", CellType::Float(self.size)),
                Cell::new_title("DOB", CellType::Date(self.date)),
                Cell::new_title("DeletedOn", CellType::DateOpt(self.removed_on)),
            ]
        }
    }

    #[test]
    fn fr_linux() {
        let values = vec![
            Person {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: Some(NaiveDate::from_ymd_opt(2024, 11, 7).unwrap()),
                size: 1.79,
            },
            Person {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let csv = Csv::new_fr();
        let result = csv.serialize::<Person>(&values);
        let expected = r#""Name";"Taille";"DOB";"DeletedOn"
"A";1,790;28/01/2020;07/11/2024
"B";1,750;11/10/1950;"#;

        assert_eq!(expected, result);
    }

    #[test]
    fn en_linux() {
        let values = vec![
            Person {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: Some(NaiveDate::from_ymd_opt(2024, 11, 7).unwrap()),
                size: 1.79,
            },
            Person {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let csv = Csv::new_iso();
        let result = csv.serialize::<Person>(&values);
        let expected = r#""Name","Taille","DOB","DeletedOn"
"A",1.790,2020-01-28,2024-11-07
"B",1.750,1950-10-11,"#;

        assert_eq!(expected, result);
    }

    #[test]
    fn mode_none() {
        let values = vec![Basic {
            name: "A".to_string(),
            ..Default::default()
        }];

        let config = Config::new_unix_comma().with_mode(QuoteMode::None);
        let csv = Csv::new_iso().with_config(config);

        let result = csv.serialize::<Basic>(&values);
        let expected = r#"Name,Taille
A,0.000"#;

        assert_eq!(expected, result);
    }

    #[test]
    fn mode_all() {
        let values = vec![Basic {
            name: "A".to_string(),
            ..Default::default()
        }];

        let config = Config::new_unix_semi_column().with_mode(QuoteMode::All);

        let csv = Csv::new_fr().with_config(config);
        let result = csv.serialize::<Basic>(&values);
        let expected = r#""Name";"Taille"
"A";"0,000""#;

        assert_eq!(expected, result);
    }

    #[test]
    fn mode_mix() {
        let values = vec![Basic {
            name: "A".to_string(),
            ..Default::default()
        }];

        let config = Config::new_unix_semi_column();

        let csv = Csv::new_fr().with_config(config);
        let result = csv.serialize::<Basic>(&values);
        let expected = r#""Name";"Taille"
"A";0,000"#;

        assert_eq!(expected, result);
    }

    #[test]
    fn no_header() {
        let values = vec![Basic {
            name: "A".to_string(),
            ..Default::default()
        }];

        let config = Config::new_unix_semi_column().with_header(false);

        let csv = Csv::new_fr().with_config(config);
        let result = csv.serialize::<Basic>(&values);
        let expected = r#""A";0,000"#;

        assert_eq!(expected, result);
    }
}
