#[test]
fn test_function_with_args() {
    let stmt = parser::parse(r#"fn greet(name: &str) { println!("Hi, {}", name); }"#);
    let py = converter::convert(stmt);
    assert_eq!(py, r#"def greet(name):\n    print("Hi, {}", name)"#);
}

#[test]
fn test_add_function() {
    let stmt = parser::parse(r#"fn add(a: i32, b: i32) { println!("{}", a + b); }"#);
    let py = converter::convert(stmt);
    assert_eq!(py, r#"def add(a, b):\n    print(a + b)"#);
}
