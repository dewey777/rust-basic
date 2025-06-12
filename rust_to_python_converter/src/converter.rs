use crate::parser::ParsedStatement;

pub fn convert(stmt: ParsedStatement) -> String {
    match stmt.kind.as_str() {
        "struct_declaration" => {
            let name = extract_struct_name(&stmt.content);
            let fields = extract_struct_fields(&stmt.content);
            let mut py = format!("class {}:\n    def __init__(self", name);
            for (field, _) in &fields {
                py += &format!(", {}", field);
            }
            py += "):\n";
            for (field, _) in &fields {
                py += &format!("        self.{} = {}\n", field, field);
            }
            py
        }
        "enum_declaration" => {
            let name = extract_enum_name(&stmt.content);
            format!(
                "# Enum (manual conversion needed)\nclass {}:\n    pass",
                name
            )
        }
        "trait_declaration" => {
            let name = extract_trait_name(&stmt.content);
            format!(
                "from abc import ABC, abstractmethod\n\nclass {}(ABC):\n    @abstractmethod\n    def method(self):\n        pass",
                name
            )
        }
        _ => "# Unsupported statement".to_string(),
    }
}

fn extract_struct_name(content: &str) -> String {
    content.split_whitespace().nth(1).unwrap_or("Unnamed").to_string()
}

fn extract_struct_fields(content: &str) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    if let Some(start) = content.find('{') {
        if let Some(end) = content.find('}') {
            let body = &content[start + 1..end];
            for line in body.lines() {
                let line = line.trim().replace(",", "");
                if line.is_empty() {
                    continue;
                }
                if let Some((name, typ)) = line.split_once(":") {
                    fields.push((name.trim().to_string(), typ.trim().to_string()));
                }
            }
        }
    }
    fields
}

fn extract_enum_name(content: &str) -> String {
    content.split_whitespace().nth(1).unwrap_or("UnnamedEnum").to_string()
}

fn extract_trait_name(content: &str) -> String {
    content.split_whitespace().nth(1).unwrap_or("UnnamedTrait").to_string()
}
