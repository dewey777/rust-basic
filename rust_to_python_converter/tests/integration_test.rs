use rust_to_python_converter::{parser, converter};

#[test]
fn test_variable_conversion() {
    let stmt = parser::parse("let x = 42;");
    let py = converter::convert(stmt);
    assert_eq!(py, "x = 42");
}
