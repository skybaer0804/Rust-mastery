// 소유권

fn main() {
    // === 스택 vs 힙 ===
    // 스택: 고정 크기, 빠름 (정수, bool 등)
    // 힙: 가변 크기, 느림 (String 등)

    // === 이동 (Move) ===
    let s1 = String::from("hello");
    let s2 = s1; // s1의 소유권이 s2로 이동
    // println!("{}", s1); // 컴파일 에러! s1은 더 이상 유효하지 않음
    println!("s2 = {}", s2);

    // === 복사 (Copy) — 스택 데이터 ===
    let x = 5;
    let y = x; // Copy — 둘 다 유효
    println!("x = {}, y = {}", x, y);

    // === 클론 (Clone) — 힙 데이터 깊은 복사 ===
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // 둘 다 유효

    // === 함수와 소유권 ===
    let s = String::from("hello");
    takes_ownership(s); // s의 소유권이 함수로 이동
    // println!("{}", s); // 컴파일 에러!

    let x = 5;
    makes_copy(x); // i32는 Copy이므로 여전히 유효
    println!("x 여전히 유효: {}", x);

    // === 반환값과 소유권 ===
    let s1 = gives_ownership();
    println!("받은 소유권: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("{}", s2); // 컴파일 에러! 소유권이 s3로
    println!("되돌려 받은 소유권: {}", s3);
}

fn takes_ownership(some_string: String) {
    println!("소유권 가져옴: {}", some_string);
} // some_string이 drop됨

fn makes_copy(some_integer: i32) {
    println!("복사된 값: {}", some_integer);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 소유권을 반환
}
