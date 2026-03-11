// 패턴과 패턴 매칭 심화

#[derive(Debug)]
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // === 리터럴 매칭 ===
    let x = 1;
    match x {
        1 => println!("하나"),
        2 => println!("둘"),
        3 => println!("셋"),
        _ => println!("기타"),
    }

    // === 여러 패턴 (|) ===
    let x = 1;
    match x {
        1 | 2 => println!("1 또는 2"),
        3 => println!("셋"),
        _ => println!("기타"),
    }

    // === 범위 패턴 (..=) ===
    let x = 5;
    match x {
        1..=5 => println!("1~5 사이"),
        _ => println!("기타"),
    }

    let c = 'c';
    match c {
        'a'..='j' => println!("앞쪽 알파벳"),
        'k'..='z' => println!("뒤쪽 알파벳"),
        _ => println!("기타"),
    }

    // === 구조체 구조분해 ===
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("x축 위: x={}", x),
        Point { x: 0, y } => println!("y축 위: y={}", y),
        Point { x, y } => println!("({}, {})", x, y),
    }

    // === Enum 구조분해 ===
    let msg = Command::ChangeColor(0, 160, 255);

    match msg {
        Command::Quit => println!("종료"),
        Command::Echo(text) => println!("에코: {}", text),
        Command::Move { x, y } => println!("이동: ({}, {})", x, y),
        Command::ChangeColor(r, g, b) => println!("색상: ({}, {}, {})", r, g, b),
    }

    // === 중첩 구조분해 ===
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("키: {}'{}\", 위치: ({}, {})", feet, inches, x, y);

    // === _ 패턴 — 값 무시 ===
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("일부 값: {}, {}, {}", first, third, fifth);
        }
    }

    // 변수명 앞 _ — 바인딩하지만 미사용 경고 방지
    let _x = 5;

    // === .. 패턴 — 나머지 무시 ===
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x = {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("처음: {}, 마지막: {}", first, last),
    }

    // === 매치 가드 (match guard) ===
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("{}: 짝수", x),
        Some(x) => println!("{}: 홀수", x),
        None => (),
    }

    // 매치 가드와 | 패턴 조합
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // (4|5|6) if y
        _ => println!("no"),
    }

    // === @ 바인딩 ===
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7, // 범위 확인 + 변수 바인딩 동시에
        } => println!("범위 내 id: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("범위 내 (변수 없음)"),
        Message::Hello { id } => println!("기타 id: {}", id),
    }

    // === if let 체이닝 ===
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("좋아하는 색: {}", color);
    } else if is_tuesday {
        println!("화요일엔 초록!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("보라색 사용");
        } else {
            println!("주황색 사용");
        }
    } else {
        println!("파란색 사용");
    }
}

enum Message {
    Hello { id: i32 },
}
