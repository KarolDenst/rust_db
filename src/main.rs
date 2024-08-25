// mod db;
mod lang;

use lang::grammar::program::Program;
use lang::lexer::Lexer;

fn main() {
    let source = "
            CREATE TABLE test (id INT, name VARCHAR(255), active BOOL);
        "
    .to_string();

    let mut scanner = Lexer::new(source);
    let tokens = scanner.get_tokens();
    let ast = Program::parse_tokens(&tokens);

    println!("{:?}", ast);
}
