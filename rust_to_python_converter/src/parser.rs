pub struct ParsedStatement {
    pub kind: String,
    pub content: String,
}

pub fn parse_statement(input: &str) -> ParsedStatement {
    let trimmed = input.trim();

    if trimmed.starts_with("struct") {
        ParsedStatement {
            kind: "struct_declaration".to_string(),
            content: trimmed.to_string(),
        }
    } else if trimmed.starts_with("enum") {
        ParsedStatement {
            kind: "enum_declaration".to_string(),
            content: trimmed.to_string(),
        }
    } else if trimmed.starts_with("trait") {
        ParsedStatement {
            kind: "trait_declaration".to_string(),
            content: trimmed.to_string(),
        }
    } else {
        ParsedStatement {
            kind: "unknown".to_string(),
            content: trimmed.to_string(),
        }
    }
}
