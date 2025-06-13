fn main() {
    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = false;
    let d: char = '💡';
    let e: &str = "Hello Rust!";

    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    let sum = add(5, 7);

    println!("{}",sum);

    if sum > 12{
        println!("같거나 크다");
    }else{
        println!(
            "작더라"
        );
    }

    let grade = match sum{
        0..=5 => "F",
        6..=8 => "D",
        9..=10 => "C",
        11..=13 => "B",
        _ => "A",
    };

    println!("grade: {}", grade);


}

fn add(x: i32, y: i32) -> i32 {
    x + y
}