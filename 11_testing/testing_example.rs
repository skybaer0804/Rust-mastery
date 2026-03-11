// 테스트 (Testing)
// rustc --test testing_example.rs && ./testing_example 로 실행

// 테스트 대상 함수들
fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

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
}

// 테스트 모듈
#[cfg(test)]
mod tests {
    use super::*; // 부모 모듈의 모든 항목 가져오기

    #[test]
    fn it_adds_two() {
        // assert_eq!로 값 비교
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_ne() {
        // assert_ne!로 값이 다른지 확인
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        // assert!로 조건 확인
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // 커스텀 에러 메시지
        assert!(
            result.contains("Carol"),
            "Greeting에 이름이 포함되지 않음, 값: `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "1~100 사이")]
    fn greater_than_100() {
        // should_panic: panic이 발생해야 테스트 통과
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        // Result를 반환하는 테스트 — ? 연산자 사용 가능
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2가 4가 아닙니다"))
        }
    }
}

fn main() {
    println!("테스트는 `rustc --test testing_example.rs && ./testing_example` 로 실행하세요.");
    println!("또는 일반 실행: add_two(3) = {}", add_two(3));
}
