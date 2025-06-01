pub struct ParsedStatement {
    pub kind: String,
    pub content: String,
}

pub fn parse(code: &str) -> ParsedStatement {
    let trimmed = code.trim();

    if trimmed.starts_with("let") {
        ParsedStatement {
            kind: "variable_declaration".to_string(),
            content: trimmed.to_string(),
        }
    } else if trimmed.starts_with("fn") {
        ParsedStatement {
            kind: "function_definition".to_string(),
            content: trimmed.to_string(),
        }
    } else {
        ParsedStatement {
            kind: "unknown".to_string(),
            content: trimmed.to_string(),
        }
    }
}
