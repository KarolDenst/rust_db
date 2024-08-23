mod db;
mod lang;

use lang::lexer::Lexer;

fn main() {
    let source = "
            CREATE TABLE test (id INT, name VARCHAR(255), active BOOL);
        "
    .to_string();

    let mut scanner = Lexer::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}
