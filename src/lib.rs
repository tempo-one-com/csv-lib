use chrono::NaiveDate;
use csv::{Cell, CsvStruct};

mod csv;
mod formater;
mod lang;

use csv::CellType;


#[cfg(test)]
mod tests {

    use csv::*;
    use formater::{FormatEn, FormatFr};

    use super::*;

    #[derive(Debug, Default)]
    struct Test {
        name: String,
        date: NaiveDate,
        removed_on: Option<NaiveDate>,
        size: f32,
    }
    
    impl CsvStruct for Test {
        fn get_cells(&self) -> Vec<csv::Cell> {
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
            Test {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: Some(NaiveDate::from_ymd_opt(2024, 11, 7).unwrap()),
                size: 1.79,
            },
            Test {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let config = Config::new_unix_fr();
        let formater = FormatFr;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values);
        let expected = r#""Name";"Taille";"DOB";"DeletedOn"
"A";1,790;28/01/2020;07/11/2024
"B";1,750;11/10/1950;"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn en_linux() {
        let values = vec![
            Test {
                name: "A".to_string(),
                date: NaiveDate::from_ymd_opt(2020, 1, 28).unwrap(),
                removed_on: Some(NaiveDate::from_ymd_opt(2024, 11, 7).unwrap()),
                size: 1.79,
            },
            Test {
                name: "B".to_string(),
                date: NaiveDate::from_ymd_opt(1950, 10, 11).unwrap(),
                removed_on: None,
                size: 1.75,
            },
        ];

        let config = Config {
            eol: Eol::Unix,
            separator: FieldSeparator::Comma,
            mode: QuoteMode::Mixed,
        };
        let formater = FormatEn;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values);
        let expected = r#""Name","Taille","DOB","DeletedOn"
"A",1.790,2020-01-28,2024-11-07
"B",1.750,1950-10-11,"#;

        assert_eq!(result, expected);
    }

    #[test]
    fn mode_none() {
        let values = vec![
            Test {
                name: "A".to_string(),
                ..Default::default()
            },
        ];

        let config = Config::new_unix_fr();
        let formater = FormatFr;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values);
        let expected = r#""Name";"Taille";"DOB";"DeletedOn"
"A";0,000;01/01/1970;"#;

        assert_eq!(result, expected);
    }

    #[test]    
    fn mode_all() {
        let values = vec![
            Test {
                name: "A".to_string(),
                ..Default::default()
            },
        ];

        let config = Config::new_unix_fr();
        let formater = FormatFr;

        let csv = Csv { config, formater };
        let result = csv.serialize::<_, Test>(&values);
        let expected = r#""Name";"Taille";"DOB";"DeletedOn"
"A";0,000;01/01/1970;"#;

        assert_eq!(result, expected);
    }

}
