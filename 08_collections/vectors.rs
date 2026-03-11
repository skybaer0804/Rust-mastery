// Vec<T> — 벡터

fn main() {
    // === 생성 ===
    let v1: Vec<i32> = Vec::new(); // 빈 벡터 (타입 명시)
    let v2 = vec![1, 2, 3];       // vec! 매크로 (타입 추론)
    println!("{:?}, {:?}", v1, v2);

    // === 요소 추가 ===
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("push 후: {:?}", v);

    // === 요소 접근 ===
    let third: &i32 = &v[2];      // 인덱스 (범위 초과 시 panic)
    println!("세 번째: {}", third);

    let third: Option<&i32> = v.get(2); // get (범위 초과 시 None)
    match third {
        Some(value) => println!("세 번째 (get): {}", value),
        None => println!("세 번째 요소 없음"),
    }

    // 범위 초과 안전 처리
    let does_not_exist = v.get(100);
    println!("존재하지 않는 요소: {:?}", does_not_exist); // None

    // === 순회 ===
    // 불변 순회
    let v = vec![100, 32, 57];
    for i in &v {
        println!("불변 순회: {}", i);
    }

    // 가변 순회
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 역참조로 값 변경
    }
    println!("가변 순회 후: {:?}", v);

    // === Enum으로 여러 타입 저장 ===
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("정수: {}", i),
            SpreadsheetCell::Float(f) => println!("실수: {}", f),
            SpreadsheetCell::Text(s) => println!("텍스트: {}", s),
        }
    }
}
