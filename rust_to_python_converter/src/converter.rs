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

            // 함수 이름 추출
            let name_start = code.find("fn ").unwrap() + 3;
            let name_end = code[name_start..].find('(').unwrap() + name_start;
            let name = &code[name_start..name_end].trim();

            // 파라미터 추출
            let param_start = name_end + 1;
            let param_end = code[param_start..].find(')').unwrap() + param_start;
            let param_str = &code[param_start..param_end];
            let params: Vec<String> = param_str
                .split(',')
                .map(|p| p.split(':').next().unwrap().trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let python_params = params.join(", ");

            // 함수 본문 추출
            let body_start = code.find('{').unwrap() + 1;
            let body_end = code.rfind('}').unwrap();
            let body = &code[body_start..body_end].trim();

            // println! 처리
            let python_body = if body.contains("println!") {
                let print_start = body.find('!').unwrap() + 1;
                let content = &body[print_start..]
                    .trim()
                    .trim_start_matches('(')
                    .trim_end_matches(')')
                    .trim_matches('"');
                // `"Hello, {}"` 와 변수
                let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

                if parts.len() >= 2 {
                    let fmt = parts[0];
                    let mut i = 1;
                    let f_string = fmt.replace("{}", &format!("{{{}}}", parts[i]));
                    format!("print(f\"{}\")", f_string)
                } else {
                    format!("print(\"{}\")", content)
                }
            } else {
                body.to_string()
            };

            format!("def {}({}):\n    {}", name, python_params, python_body)
        }

        _ => "# 지원하지 않는 구문입니다.".to_string(),
    }
}
