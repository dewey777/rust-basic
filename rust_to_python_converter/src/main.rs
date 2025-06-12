mod converter;
mod parser;

use std::env;
use parser::parse_statement;
use converter::convert;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- '<rust code snippet>'");
        return;
    }

    let input = &args[1];
    let parsed = parse_statement(input);
    let python_code = convert(parsed);
    println!("{}", python_code);
}
