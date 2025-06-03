#[cfg(test)]
mod tests {
    use crate::converter::*;
    use crate::parser::ParsedStatement;

    #[test]
    fn test_println_to_fstring() {
        let stmt = ParsedStatement {
            kind: "function_definition".to_string(),
            content: "fn greet(name: &str) { println!(\"Hello, {}\", name); }".to_string(),
        };
        let py = convert(stmt);
        assert_eq!(py, "def greet(name):\n    print(f\"Hello, {name}\")");
    }

    #[test]
    fn test_let_statement() {
        let stmt = ParsedStatement {
            kind: "variable_declaration".to_string(),
            content: "let x = 5;".to_string(),
        };
        let py = convert(stmt);
        assert_eq!(py, "x = 5");
    }
}
