// 참조와 빌림 (References and Borrowing)

fn main() {
    // === 불변 참조 (&) ===
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 참조 전달 (소유권 이동 없음)
    println!("'{}'의 길이: {}", s1, len); // s1 여전히 유효!

    // 여러 불변 참조 동시 가능
    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);

    // === 가변 참조 (&mut) ===
    let mut s = String::from("hello");
    change(&mut s);
    println!("변경 후: {}", s);

    // 가변 참조는 한 번에 하나만!
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // 컴파일 에러! 동시에 두 개의 가변 참조 불가
    println!("가변 참조: {}", r1);

    // 불변 참조와 가변 참조 동시 사용 불가
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // r1, r2는 여기서 마지막 사용 (NLL: Non-Lexical Lifetimes)
    let r3 = &mut s; // OK — r1, r2가 더 이상 사용되지 않으므로
    println!("{}", r3);

    // === 댕글링 참조 방지 ===
    // fn dangle() -> &String { // 컴파일 에러!
    //     let s = String::from("hello");
    //     &s // s가 drop되므로 댕글링 참조
    // }
    // 대신 소유권을 반환:
    let s = no_dangle();
    println!("no_dangle: {}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s는 참조이므로 drop되지 않음

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 소유권 이동 (댕글링 없음)
}
