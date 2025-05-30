use crate::parser::ParsedStatement;

pub fn convert(stmt: ParsedStatement) -> String {
    match stmt.kind.as_str() {
        "variable_declaration" => {
            // "let x = 5;" → "x = 5"
            let cleaned = stmt.content
                .replace("let ", "")
                .replace(";", "");  // 💡 중간 값으로 보존

            let parts: Vec<&str> = cleaned
                .split('=')
                .map(|s| s.trim())
                .collect();

            if parts.len() == 2 {
                format!("{} = {}", parts[0], parts[1])
            } else {
                "# 변환 실패: 구문 오류".to_string()
            }
        }
        _ => "# 지원하지 않는 구문입니다.".to_string(),
    }
}
