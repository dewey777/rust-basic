use rust_to_python_converter::{parser, converter};

#[test]
fn test_variable_conversion() {
    let stmt = parser::parse("let x = 42;");
    let py = converter::convert(stmt);
    assert_eq!(py, "x = 42");
}

#[test]
fn test_function_conversion() {
    let stmt = parser::parse(r#"fn greet() { println!("hello"); }"#);
    let py = converter::convert(stmt);
    assert_eq!(py, "def greet():\n    print(\"hello\")");
}
