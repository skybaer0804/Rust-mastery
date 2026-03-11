// HashMap<K, V>

use std::collections::HashMap;

fn main() {
    // === 생성 및 삽입 ===
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // === 접근 ===
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // Option<&V> 반환
    match score {
        Some(s) => println!("Blue 팀 점수: {}", s),
        None => println!("팀을 찾을 수 없음"),
    }

    // copied()와 unwrap_or()
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue 팀 점수: {}", score);

    // === 순회 ===
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // === 덮어쓰기 ===
    scores.insert(String::from("Blue"), 25);
    println!("덮어쓰기 후: {:?}", scores);

    // === 없을 때만 삽입 (entry) ===
    scores.entry(String::from("Yellow")).or_insert(100); // 이미 있으므로 무시
    scores.entry(String::from("Green")).or_insert(100);  // 없으므로 삽입
    println!("entry 후: {:?}", scores);

    // === 기존 값 기반 업데이트 ===
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1; // 역참조로 값 변경
    }
    println!("단어 빈도: {:?}", word_count);

    // === 소유권 ===
    let field_name = String::from("color");
    let field_value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name); // 에러! 소유권이 HashMap으로 이동됨
    println!("소유권 이동 후: {:?}", map);
}
