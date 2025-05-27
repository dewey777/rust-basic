use rand::Rng;

pub fn generate_question() -> String {
    let questions = [
        "Rust에서 소유권이란 무엇인가요?",
        "Python과 Rust의 메모리 관리 차이점은?",
        "match 문은 무엇을 위해 사용되나요?",
        "struct는 어떤 상황에서 사용하나요?",
        "Rust에서 가변 참조는 어떻게 하나요?",
    ];

    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..questions.len());

    questions[idx].to_string()
}
