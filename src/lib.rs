pub mod cell;
pub mod config;
pub mod csv;
pub mod formatter;
pub mod models;

#[cfg(test)]
mod tests {

    use cell::Cell;
    use chrono::{NaiveDate, NaiveDateTime};
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
                Cell::new_title("Name", self.name.clone().into()),
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
        age: i32,
        date_time: NaiveDateTime,
    }

    impl CellsBuilder for Person {
        fn get_cells(&self) -> Vec<Cell> {
            vec![
                Cell::new_title("Name", self.name.clone().into()),
                Cell::new_title("Taille", self.size.into()),
                Cell::new_title("DOB", self.date.into()),
                Cell::new_title("DeletedOn", self.removed_on.into()),
                Cell::new_title("Age", self.age.into()),
                Cell::new_title("DateTime", self.date_time.into()),
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
                age: 30,
                date_time: NaiveDate::from_ymd_opt(2020, 1, 28)
                    .unwrap()
                    .and_hms_opt(10, 10, 0)
                    .unwrap(),
            },
            Person {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
                age: 25,
                date_time: NaiveDate::from_ymd_opt(1950, 10, 11)
                    .unwrap()
                    .and_hms_opt(10, 10, 0)
                    .unwrap(),
            },
        ];

        let csv = Csv::new_fr();
        let result = csv.serialize::<Person>(&values);
        let expected = r#""Name";"Taille";"DOB";"DeletedOn";"Age";"DateTime"
"A";1,790;28/01/2020;07/11/2024;30;28/01/2020T10:10
"B";1,750;11/10/1950;;25;11/10/1950T10:10"#;

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
                age: 20,
                date_time: NaiveDate::from_ymd_opt(2020, 1, 28)
                    .unwrap()
                    .and_hms_opt(10, 10, 0)
                    .unwrap(),
            },
            Person {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
                age: 20,
                date_time: NaiveDate::from_ymd_opt(1950, 10, 11)
                    .unwrap()
                    .and_hms_opt(10, 10, 0)
                    .unwrap(),
            },
        ];

        let csv = Csv::new_iso();
        let result = csv.serialize::<Person>(&values);
        let expected = r#""Name","Taille","DOB","DeletedOn","Age","DateTime"
"A",1.790,2020-01-28,2024-11-07,20,2020-01-28T10:10
"B",1.750,1950-10-11,,20,1950-10-11T10:10"#;

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
