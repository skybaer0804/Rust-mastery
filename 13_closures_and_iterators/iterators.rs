// 이터레이터 (Iterators)

fn main() {
    // === 기본 이터레이터 ===
    let v1 = vec![1, 2, 3];

    // iter(): 불변 참조 이터레이터
    let mut v1_iter = v1.iter();
    println!("next: {:?}", v1_iter.next()); // Some(&1)
    println!("next: {:?}", v1_iter.next()); // Some(&2)
    println!("next: {:?}", v1_iter.next()); // Some(&3)
    println!("next: {:?}", v1_iter.next()); // None

    // === 소비 어댑터 (consuming adaptors) ===
    let v1 = vec![1, 2, 3];
    let total: i32 = v1.iter().sum();
    println!("합계: {}", total);

    let count = v1.iter().count();
    println!("개수: {}", count);

    // === 이터레이터 어댑터 (iterator adaptors) ===
    // 지연 평가(lazy) — collect() 같은 소비 어댑터 호출 전까지 실행 안 됨

    // map: 각 요소 변환
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("map: {:?}", v2); // [2, 3, 4]

    // filter: 조건에 맞는 요소만
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<&i32> = v1.iter().filter(|x| *x % 2 == 0).collect();
    println!("짝수: {:?}", evens); // [2, 4, 6]

    // 체이닝
    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = v
        .iter()
        .filter(|&&x| x > 2)
        .map(|&x| x * 10)
        .collect();
    println!("체이닝 (>2 후 *10): {:?}", result); // [30, 40, 50]

    // enumerate: 인덱스 포함
    let v = vec!["a", "b", "c"];
    for (i, val) in v.iter().enumerate() {
        println!("인덱스 {}: {}", i, val);
    }

    // zip: 두 이터레이터 결합
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let people: Vec<(&str, &i32)> = names.iter().copied().zip(ages.iter()).collect();
    println!("zip: {:?}", people);

    // === 커스텀 이터레이터 ===
    let counter = Counter::new();
    let values: Vec<u32> = counter.collect();
    println!("Counter: {:?}", values); // [1, 2, 3, 4, 5]

    // 커스텀 이터레이터 체이닝
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("Counter 체이닝 합계: {}", sum);

    // === for 루프는 이터레이터 사용 ===
    let v = vec![10, 20, 30];
    for val in &v {
        print!("{} ", val);
    }
    println!();
}

// 커스텀 이터레이터 구현
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // 연관 타입

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
