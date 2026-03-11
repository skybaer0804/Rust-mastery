// 구조체 (Structs)

// 기본 구조체
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 유닛 구조체 (필드 없음)
struct AlwaysEqual;

fn main() {
    // 구조체 인스턴스 생성
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("user123"),
        active: true,
        sign_in_count: 1,
    };
    println!("사용자: {}", user1.username);

    // 가변 인스턴스 — 개별 필드가 아닌 전체가 가변
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("changed@example.com");
    println!("변경된 이메일: {}", user2.email);

    // 구조체 업데이트 문법 (..)
    let user3 = User {
        email: String::from("new@example.com"),
        ..user1 // 나머지 필드를 user1에서 가져옴
        // 주의: user1.username은 이동됨 (String은 Move)
    };
    println!("user3: {} (active: {})", user3.email, user3.active);

    // 필드 초기화 축약
    let user4 = build_user(
        String::from("short@example.com"),
        String::from("short"),
    );
    println!("user4: {}", user4.username);

    // 튜플 구조체
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // 유닛 구조체
    let _subject = AlwaysEqual;
}

// 필드 초기화 축약 — 매개변수명과 필드명이 같을 때
fn build_user(email: String, username: String) -> User {
    User {
        email,     // email: email 대신
        username,  // username: username 대신
        active: true,
        sign_in_count: 1,
    }
}
