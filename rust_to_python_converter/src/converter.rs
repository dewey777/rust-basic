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
            let code = stmt.content;

            let name_start = code.find("fn ").unwrap() + 3;
            let name_end = code[name_start..].find('(').unwrap() + name_start;
            let name = &code[name_start..name_end].trim();

            let param_start = name_end + 1;
            let param_end = code[param_start..].find(')').unwrap() + param_start;
            let param_str = &code[param_start..param_end];

            let params: Vec<String> = param_str
                .split(',')
                .map(|p| {
                    p.trim()
                        .split(':')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string()
                })
                .filter(|s| !s.is_empty())
                .collect();
            let python_params = params.join(", ");

            let body_start = code.find('{').unwrap() + 1;
            let body_end = code.rfind('}').unwrap_or(code.len());
            let body = &code[body_start..body_end].trim();

            let python_body = body
                .replace("println!", "print")
                .replace(";", "")
                .replace("{}", "{}");

            format!("def {}({}):\n    {}", name, python_params, python_body)
        }

        _ => "# 지원하지 않는 구문입니다.".to_string(),
    }
}
