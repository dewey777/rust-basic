use rand::Rng;

pub fn generate_question() -> String {
    let questions = [
        "Rust의 소유권이란?",
        "match 문이란?",
        "struct는 언제 사용?",
        "라이프타임이란?",
        "Result와 Option의 차이?",
    ];
    let mut rng = rand::thread_rng();
    let idx = rng.gen_range(0..questions.len());
    questions[idx].to_string()
}

// 유닛 테스트
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_not_empty() {
        let q = generate_question();
        assert!(!q.is_empty());
    }
}
