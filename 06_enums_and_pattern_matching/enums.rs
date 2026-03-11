// Enum (열거형)

// 기본 enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 데이터를 포함하는 enum
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 다양한 타입의 데이터를 포함하는 enum
#[derive(Debug)]
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 익명 구조체
    Write(String),              // String
    ChangeColor(i32, i32, i32), // 튜플
}

// enum에 메서드 정의 가능
impl Message {
    fn call(&self) {
        println!("메시지 호출: {:?}", self);
    }
}

fn main() {
    // 기본 enum 사용
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}, {:?}", four, six);

    // 데이터 포함 enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    // 메서드 호출
    let m = Message::Write(String::from("hello"));
    m.call();

    // === Option<T> — null 대신 사용 ===
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None; // 타입 명시 필요

    println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);

    // Option 값 사용 — 직접 연산 불가, 꺼내야 함
    let x: i32 = 5;
    let y: Option<i32> = Some(5);
    // let sum = x + y; // 컴파일 에러! i32와 Option<i32>는 다른 타입
    let sum = x + y.unwrap_or(0); // unwrap_or로 기본값 제공
    println!("합계: {}", sum);
}
