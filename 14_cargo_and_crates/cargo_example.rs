// Cargo와 Crates — 개념 설명용 예제

/// 두 수를 더합니다.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 숫자가 짝수인지 확인합니다.
///
/// # Examples
///
/// ```
/// assert!(is_even(4));
/// assert!(!is_even(3));
/// ```
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 재사용 가능한 모듈 구조 예시
mod math {
    pub mod basic {
        /// 절대값을 반환합니다.
        pub fn abs(x: i32) -> i32 {
            if x < 0 { -x } else { x }
        }

        /// 최대값을 반환합니다.
        pub fn max(a: i32, b: i32) -> i32 {
            if a > b { a } else { b }
        }
    }
}

// pub use로 재내보내기 (API 구조 단순화)
use math::basic;

fn main() {
    println!("=== Cargo & Crates 개념 예제 ===");
    println!();

    // 문서 주석이 있는 함수 사용
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(3) = {}", is_even(3));

    // 모듈 사용
    println!("abs(-5) = {}", basic::abs(-5));
    println!("max(3, 7) = {}", basic::max(3, 7));

    println!();
    println!("=== Cargo 주요 명령어 ===");
    println!("cargo new <name>      : 새 프로젝트 생성");
    println!("cargo build           : 빌드 (dev 프로파일)");
    println!("cargo build --release : 릴리스 빌드 (최적화)");
    println!("cargo run             : 빌드 + 실행");
    println!("cargo test            : 테스트 실행");
    println!("cargo doc --open      : 문서 생성 및 열기");
    println!("cargo publish         : crates.io에 배포");
}
