// 함수

fn main() {
    // 함수 호출
    another_function();
    print_value(5);
    print_labeled_measurement(5, 'h');

    // 반환값이 있는 함수
    let x = five();
    println!("five() = {}", x);

    let y = plus_one(5);
    println!("plus_one(5) = {}", y);

    // 표현식 vs 구문
    // 구문(statement): 값을 반환하지 않음
    // 표현식(expression): 값을 반환함
    let z = {
        let x = 3;
        x + 1 // 세미콜론 없음 → 표현식 (값 반환)
    };
    println!("블록 표현식: {}", z); // 4
}

// 매개변수 없는 함수
fn another_function() {
    println!("다른 함수입니다.");
}

// 매개변수 — 타입 명시 필수
fn print_value(x: i32) {
    println!("x의 값: {}", x);
}

// 여러 매개변수
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("측정값: {}{}", value, unit_label);
}

// 반환값 — 마지막 표현식이 암묵적 반환값
fn five() -> i32 {
    5 // 세미콜론 없음!
}

fn plus_one(x: i32) -> i32 {
    x + 1 // 세미콜론 붙이면 구문이 되어 컴파일 에러
}
