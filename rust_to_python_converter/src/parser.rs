pub struct ParsedStatement {
    pub kind: String,
    pub content: String,
}

pub fn parse(code: &str) -> ParsedStatement {
    // 실제 파서 대신 간단한 더미 파싱
    if code.trim().starts_with("let") {
        ParsedStatement {
            kind: "variable_declaration".to_string(),
            content: code.trim().to_string(),
        }
    } else {
        ParsedStatement {
            kind: "unknown".to_string(),
            content: code.trim().to_string(),
        }
    }
}
