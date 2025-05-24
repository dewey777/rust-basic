fn main() {
    println!("▶ Day 3: 제어 흐름, match, enum, struct 실습");

    // 1. 조건문
    let number = 7;
    if number % 2 == 0 {
        println!("짝수입니다");
    } else if number % 3 == 0 {
        println!("3의 배수입니다");
    } else {
        println!("홀수입니다");
    }

    // 2. 반복문
    let mut count = 0;
    loop {
        count += 1;
        println!("loop 반복: {}", count);
        if count == 3 {
            break;
        }
    }

    // 3. for 문
    for i in 1..=5 {
        println!("for 루프 i = {}", i);
    }

    // 4. match 문
    let lang = "Rust";
    match lang {
        "Rust" => println!("🦀 러스트를 배우는 중입니다!"),
        "Python" => println!("🐍 파이썬도 좋아요"),
        _ => println!("다른 언어입니다"),
    }

    // 5. enum 사용
    #[derive(Debug)]
    enum Language {
        Rust,
        Python,
        JavaScript,
    }

    let my_lang = Language::Python;
    match my_lang {
        Language::Rust => println!("러스트 개발자"),
        Language::Python => println!("파이썬 개발자"),
        Language::JavaScript => println!("자바스크립트 개발자"),
    }

    // 6. struct 정의 및 사용
    struct Developer {
        name: String,
        language: Language,
    }

    let dev = Developer {
        name: String::from("Alex"),
        language: Language::Rust,
    };

    println!("{}는 {:?} 개발자입니다.", dev.name, dev.language);
}
