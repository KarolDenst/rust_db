pub struct Table {
    pub name: String,
    pub columns: Vec<Type>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new(name: &str, columns: Vec<Type>) -> Table {
        Table {
            name: name.to_string(),
            columns,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, data: Vec<Data>) -> Result<(), String> {
        self.rows.push(Row { data });
        return Ok(());
    }
}

pub struct Row {
    pub data: Vec<Data>,
}

pub enum Data {
    Int(i32),
    Float(f32),
    Varchar(String),
    Bool(bool),
}

pub enum Type {
    Int,
    Float,
    Varchar { length: u16 },
    Bool,
}
