use crate::parser::ParsedStatement;

pub fn convert(stmt: ParsedStatement) -> String {
    match stmt.kind.as_str() {
        "variable_declaration" => {
            let cleaned = stmt.content
                .replace("let ", "")
                .replace(";", "");

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
        "function_definition" => {
            // 예: fn greet() { println!("hi"); }
            // → def greet():\n    print("hi")

            let code = stmt.content;

            // 함수명 추출
            let name_start = code.find("fn ").unwrap() + 3;
            let name_end = code[name_start..].find('(').unwrap() + name_start;
            let name = &code[name_start..name_end];

            // 내부 내용 추출
            let body_start = code.find('{').unwrap() + 1;
            let body_end = code.rfind('}').unwrap_or(code.len());
            let body = &code[body_start..body_end].trim();

            // `println!("hi");` → `print("hi")`
            let python_body = body
                .replace("println!", "print")
                .replace(";", "");

            format!("def {}():\n    {}", name.trim(), python_body.trim())
        }
        _ => "# 지원하지 않는 구문입니다.".to_string(),
    }
}
