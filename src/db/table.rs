use crate::lang::grammar::{column_def::ColumnDef, row::Row};

pub struct Table {
    pub name: String,
    pub columns: Vec<ColumnDef>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: &str, columns: Vec<ColumnDef>) -> Table {
        Table {
            name: name.to_string(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, data: Vec<Row>) -> Result<(), String> {
        // TODO: Validate data against columns
        self.rows.extend(data);
        return Ok(());
    }

    pub fn select(&self) -> Result<(), String> {
        for (i, val) in self.columns.iter().enumerate() {
            if i > 0 {
                print!("|");
            }
            print!("{}", val);
        }
        println!();
        for row in &self.rows {
            println!("{}", row);
        }
        return Ok(());
    }
}
