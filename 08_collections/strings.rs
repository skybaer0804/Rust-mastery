// String — 문자열

fn main() {
    // === 생성 ===
    let mut s1 = String::new();                    // 빈 문자열
    let s2 = "initial contents".to_string();       // &str → String
    let s3 = String::from("initial contents");     // String::from
    println!("{:?}, {}, {}", s1, s2, s3);

    // === 문자열 추가 ===
    // push_str: 문자열 슬라이스 추가
    s1.push_str("hello");
    println!("push_str: {}", s1);

    let s = String::from(" world");
    s1.push_str(&s); // &str로 빌림 (소유권 이동 없음)
    println!("push_str: {}, s 여전히 유효: {}", s1, s);

    // push: 단일 문자 추가
    s1.push('!');
    println!("push: {}", s1);

    // === 문자열 결합 ===
    // + 연산자 (add 메서드: fn add(self, s: &str) -> String)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 이동됨, s2는 빌림
    // println!("{}", s1); // 에러! s1은 이동됨
    println!("+ 연산자: {}", s3);

    // format! 매크로 (소유권 이동 없음)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    println!("s1 여전히 유효: {}", s1); // OK!

    // === UTF-8과 인덱싱 ===
    let hello = String::from("안녕하세요");
    println!("바이트 길이: {}", hello.len()); // 15 (한글은 3바이트)
    println!("문자 수: {}", hello.chars().count()); // 5

    // s[0]은 불가! — UTF-8에서 바이트 인덱스와 문자가 일치하지 않으므로

    // 문자 순회
    for c in "안녕".chars() {
        println!("문자: {}", c);
    }

    // 바이트 순회
    for b in "안녕".bytes() {
        println!("바이트: {}", b);
    }

    // 문자열 슬라이스 (바이트 경계 주의!)
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 첫 2글자 (각 2바이트)
    println!("슬라이스: {}", s);
}
