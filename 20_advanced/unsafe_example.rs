// Unsafe Rust

fn main() {
    // === 원시 포인터 역참조 ===
    println!("=== 원시 포인터 ===");
    let mut num = 5;

    // 원시 포인터 생성은 safe 코드에서도 가능
    let r1 = &num as *const i32;    // 불변 원시 포인터
    let r2 = &mut num as *mut i32;  // 가변 원시 포인터

    // 역참조는 unsafe 블록 필요
    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
    }

    // === unsafe 함수 호출 ===
    println!("\n=== unsafe 함수 ===");
    unsafe {
        dangerous();
    }

    // === safe 추상화 (split_at_mut 구현) ===
    println!("\n=== safe 추상화 ===");
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("왼쪽: {:?}", left);
    println!("오른쪽: {:?}", right);

    // === extern 함수 (FFI) ===
    println!("\n=== FFI (C 함수 호출) ===");
    unsafe {
        println!("C abs(-3) = {}", abs(-3));
    }

    // === 가변 정적 변수 ===
    println!("\n=== 가변 정적 변수 ===");
    add_to_count(3);
    println!("COUNTER = {}", get_count());

    // === unsafe 트레이트 ===
    println!("\n=== unsafe 트레이트 ===");
    // Send, Sync 등이 대표적 — 컴파일러가 자동 구현하지만
    // 수동으로 unsafe impl 할 수 있음
    println!("Send/Sync는 자동 구현되는 unsafe 트레이트입니다.");

    println!("\nunsafe는 빌림 검사를 끄는 것이 아닙니다!");
    println!("특정 작업만 허용하며, 안전 보장 책임이 프로그래머에게 있습니다.");
}

// unsafe 함수
unsafe fn dangerous() {
    println!("이것은 unsafe 함수입니다");
}

// unsafe를 safe 함수로 감싸기 (추상화)
fn split_at_mut(values: &mut Vec<i32>, mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// FFI: C 라이브러리 함수 선언
extern "C" {
    fn abs(input: i32) -> i32;
}

// 가변 정적 변수 — 안전한 접근을 위해 포인터 사용
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        let counter = &raw mut COUNTER;
        *counter += inc;
    }
}

fn get_count() -> u32 {
    unsafe {
        let counter = &raw const COUNTER;
        *counter
    }
}
