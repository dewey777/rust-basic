mod quiz; // quiz.rs 모듈 불러오기

fn main() {
    println!("▶ Day 4: 퀴즈 생성기 시작!");

    let question = quiz::generate_question();
    println!("Q: {}", question);
}
