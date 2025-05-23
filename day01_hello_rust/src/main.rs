fn main() {
    let name = "Rust";
    let mut count = 5;

    println!("Hello, {}!", name);

    for i in 0..count {
        println!("Count: {}", i);
    }

    count = count + 1;
    println!("Final count: {}", count);

    greet("Alice");
}

fn greet(name: &str) {
    println!("Nice to meet you, {}!", name);
}