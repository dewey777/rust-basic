mod parser;
mod converter;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("❗ 사용법: cargo run -- <rust_code>");
        return;
    }

    let input_code = &args[1];
    let parsed = parser::parse(input_code);
    let python_code = converter::convert(parsed);

    println!("✅ 변환된 Python 코드:\n{}", python_code);
}
