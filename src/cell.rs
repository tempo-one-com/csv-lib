use crate::models::CellType;


pub struct Cell {
    pub title: Option<String>,
    pub value: CellType,
}

impl Cell {
    pub fn new(value: CellType) -> Self {
        Self {
            value,
            title: None,
        }
    }

    pub fn new_title(title: &str, value: CellType) -> Self {
        Self {
            title: Some(title.to_string()),
            value,
        }
    }
}