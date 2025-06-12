mod converter;
mod parser;

use std::fs;

fn main() {
    let input_path = "sample.rs";
    let output_path = "output.py";

    let source = fs::read_to_string(input_path).expect("Failed to read Rust file");

    let mut output_lines = Vec::new();

    for line in source.lines() {
        let parsed = parser::parse_statement(line);
        let python = converter::convert(parsed);
        output_lines.push(python);
    }

    let result = output_lines.join("\n\n");
    fs::write(output_path, result).expect("Failed to write Python file");

    println!("Conversion completed! Output saved to {}", output_path);
}
