mod table;

use table::{Data, Row, Table, Type};

pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new(name: &str) -> Database {
        Database {
            name: name.to_string(),
            tables: Vec::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<Type>) -> Result<(), String> {
        if self.tables.iter().any(|table| table.name == name) {
            return Err(format!("Table {} already exists", name));
        }
        self.tables.push(Table {
            name: name.to_string(),
            columns,
            rows: Vec::new(),
        });
        Ok(())
    }

    pub fn insert(&mut self, table_name: &str, data: Vec<Data>) -> Result<(), String> {
        let table = self
            .tables
            .iter_mut()
            .find(|table| table.name == table_name);
        match table {
            Some(table) => {
                table.insert(data);
                Ok(())
            }
            None => Err(format!("Table {} does not exist", table_name)),
        }
    }
}
