// 제어 흐름

fn main() {
    // === if 표현식 ===
    let number = 7;

    if number < 5 {
        println!("조건: true");
    } else {
        println!("조건: false");
    }

    // else if
    if number % 4 == 0 {
        println!("4로 나누어 떨어짐");
    } else if number % 3 == 0 {
        println!("3으로 나누어 떨어짐");
    } else if number % 2 == 0 {
        println!("2로 나누어 떨어짐");
    } else {
        println!("4, 3, 2로 나누어 떨어지지 않음");
    }

    // let 구문에서 if 사용 (삼항 연산자 대체)
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);

    // === loop — 무한 루프 ===
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break로 값 반환
        }
    };
    println!("loop 결과: {}", result); // 20

    // 루프 레이블 — 중첩 루프에서 특정 루프 break
    let mut count = 0;
    'outer: loop {
        let mut remaining = 10;
        loop {
            if remaining == 9 {
                break; // 안쪽 루프만 종료
            }
            if count == 2 {
                break 'outer; // 바깥쪽 루프 종료
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("count = {}", count); // 2

    // === while ===
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("발사!");

    // === for — 컬렉션 순회 (가장 안전하고 일반적) ===
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("값: {}", element);
    }

    // 범위(Range) 사용
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("발사!");
}
