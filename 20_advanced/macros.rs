// 매크로 (Macros)

// === 선언적 매크로 (macro_rules!) ===

// 간단한 vec! 매크로 재구현
macro_rules! my_vec {
    // 빈 벡터
    () => {
        Vec::new()
    };
    // 요소가 있는 벡터
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 여러 패턴을 가진 매크로
macro_rules! say {
    (hello) => {
        println!("안녕하세요!")
    };
    (bye) => {
        println!("안녕히 가세요!")
    };
    ($name:expr) => {
        println!("{}님, 반갑습니다!", $name)
    };
}

// 반복 패턴 매크로
macro_rules! hash_map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

// 계산 매크로
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: f64 = $e as f64;
            println!("{} = {}", stringify!($e), val);
        }
    };
}

fn main() {
    println!("=== 선언적 매크로 (macro_rules!) ===\n");

    // my_vec! 매크로
    let v1: Vec<i32> = my_vec!();
    let v2 = my_vec![1, 2, 3];
    println!("my_vec!() = {:?}", v1);
    println!("my_vec![1,2,3] = {:?}", v2);

    // say! 매크로
    say!(hello);
    say!(bye);
    say!("Rust");

    // hash_map! 매크로
    let scores = hash_map! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    println!("\n점수: {:?}", scores);

    // calculate! 매크로
    println!();
    calculate!(eval 1 + 2);
    calculate!(eval (1 + 2) * (3 + 4));

    // === 절차적 매크로 설명 ===
    println!("\n=== 절차적 매크로 (개념) ===");
    println!("1. derive 매크로: #[derive(Debug, Clone)]");
    println!("   - 트레이트 구현 코드를 자동 생성");
    println!("2. 속성형 매크로: #[route(GET, \"/\")]");
    println!("   - 함수/구조체에 어트리뷰트 추가");
    println!("3. 함수형 매크로: sql!(SELECT * FROM users)");
    println!("   - 함수 호출처럼 사용하지만 컴파일 타임에 실행");

    // 표준 derive 매크로 예시
    println!("\n=== derive 매크로 예시 ===");

    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone(); // Clone derive
    println!("{:?}", p1);  // Debug derive
    println!("p1 == p2: {}", p1 == p2); // PartialEq derive
}
