// panic! — 복구 불가능한 에러

fn main() {
    // === 기본 사용법 ===
    // panic!("crash and burn"); // 프로그램 즉시 종료

    // === 배열 범위 초과 — 자동 panic ===
    // let v = vec![1, 2, 3];
    // v[99]; // panic! index out of bounds

    // === 실용적 예시: 유효성 검증 ===
    let value = 42;
    if value < 0 || value > 100 {
        panic!("값이 범위를 벗어남: {}", value);
    }
    println!("유효한 값: {}", value);

    // === 백트레이스 확인 방법 ===
    // RUST_BACKTRACE=1 ./panic_example
    // RUST_BACKTRACE=full ./panic_example (상세)

    // === Guess 타입 — panic을 이용한 유효성 검증 ===
    let guess = Guess::new(50);
    println!("추측 값: {}", guess.value());

    // let bad_guess = Guess::new(200); // panic!
    println!("panic 예제 완료 (주석 해제하여 panic 테스트)");
}

// 커스텀 타입으로 유효 범위 강제
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess 값은 1~100 사이여야 합니다. 받은 값: {}", value);
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}
