mod quiz;

use std::io;

fn main() {
    println!("▶ Day 5: 사용자 입력과 테스트");

    let question = quiz::generate_question();
    println!("Q: {}", question);

    println!("당신의 답변을 입력하세요:");
    let mut user_input = String::new();

    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            println!("당신의 답변: {}", user_input.trim());
        }
        Err(e) => {
            println!("입력 오류: {}", e);
        }
    }
}
