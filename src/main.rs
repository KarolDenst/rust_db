// mod db;
mod db;
mod lang;

use db::database::Database;
use lang::grammar::program::Program;
use lang::lexer::get_tokens;

fn main() {
    let create = "
            CREATE TABLE test (id INT, name VARCHAR(255), active BOOL);
        "
    .to_string();
    let insert = "
            INSERT INTO test VALUES (1, 'test', true), (2, 'test2', false);
        "
    .to_string();
    let select = "
            SELECT * FROM test;
        "
    .to_string();

    let mut db = Database::new("test.db");
    Program::parse_tokens(&get_tokens(create)).execute(&mut db);
    Program::parse_tokens(&get_tokens(insert)).execute(&mut db);
    Program::parse_tokens(&get_tokens(select)).execute(&mut db);
}
