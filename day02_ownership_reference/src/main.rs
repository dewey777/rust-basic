fn main() {
    println!("▶ Day 2: Ownership, References, Mutability 실습");

    // 1. 소유권 이동 (Move)
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // ❌ error: s1은 더 이상 유효하지 않음
    println!("s2 = {}", s2);

    // 2. 복사 (Copy, 정수형 등은 복사됨)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // ✅ ok

    // 3. 함수로 소유권 이동
    let s3 = String::from("rust");
    takes_ownership(s3);
    // println!("{}", s3); // ❌ error: s3은 함수에 의해 move됨

    // 4. 참조자 전달 (&T)
    let s4 = String::from("reference");
    let len = calculate_length(&s4);
    println!("'{}'의 길이는 {}입니다", s4, len);

    // 5. 가변 참조자 (&mut T)
    let mut s5 = String::from("mutable");
    change(&mut s5);
    println!("변경된 문자열: {}", s5);

    // 6. 빌림 규칙 테스트
    let r1 = &s5;
    let r2 = &s5;
    println!("r1 = {}, r2 = {}", r1, r2); // ✅ 불변 참조는 여러 개 가능

    // let r3 = &mut s5; // ❌ error: 가변 참조는 불변 참조와 동시에 불가능
    // println!("{}", r3);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" updated");
}
