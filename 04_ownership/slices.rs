// 슬라이스 (Slices)

fn main() {
    // === 문자열 슬라이스 (&str) ===
    let s = String::from("hello world");

    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    println!("{} {}", hello, world);

    // 범위 축약
    let hello = &s[..5];    // 처음부터
    let world = &s[6..];    // 끝까지
    let whole = &s[..];     // 전체
    println!("{} {} {}", hello, world, whole);

    // === first_word 함수 — 슬라이스 활용 ===
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("첫 번째 단어: {}", word);

    // 문자열 리터럴은 이미 슬라이스 (&str)
    let literal = "hello world";
    let word = first_word(literal);
    println!("리터럴의 첫 단어: {}", word);

    // === 배열 슬라이스 ===
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // [2, 3]
    println!("배열 슬라이스: {:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

// &str을 받으면 String과 &str 모두 처리 가능
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
