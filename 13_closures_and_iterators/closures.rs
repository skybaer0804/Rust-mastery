// 클로저 (Closures)

fn main() {
    // === 기본 클로저 ===
    let add_one = |x: i32| -> i32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // 타입 추론 — 명시 불필요
    let add_two = |x| x + 2;
    println!("add_two(5) = {}", add_two(5));

    // 한 줄이면 중괄호 생략
    let double = |x| x * 2;
    println!("double(5) = {}", double(5));

    // === 환경 캡처 ===
    let x = 4;

    // Fn — 불변 참조로 캡처
    let equal_to_x = |z| z == x;
    println!("4 == 4? {}", equal_to_x(4));
    println!("x 여전히 유효: {}", x);

    // FnMut — 가변 참조로 캡처
    let mut list = vec![1, 2, 3];
    println!("정의 전: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // println!("{:?}", list); // 에러! 가변 참조 중에 불변 참조 불가
    borrows_mutably();
    println!("클로저 호출 후: {:?}", list);

    // FnOnce — 소유권 가져감
    let x = String::from("hello");
    let consume = move || println!("move로 캡처: {}", x);
    consume();
    // println!("{}", x); // 에러! 소유권이 클로저로 이동됨

    // === 클로저를 매개변수로 ===
    let list = vec![1, 2, 3];
    apply_to_list(&list, |item| println!("  항목: {}", item));

    // === 실용적 예시: 정렬 ===
    let mut numbers = vec![5, 2, 8, 1, 9, 3];
    numbers.sort_by(|a, b| a.cmp(b));
    println!("오름차순 정렬: {:?}", numbers);

    numbers.sort_by(|a, b| b.cmp(a));
    println!("내림차순 정렬: {:?}", numbers);

    // 구조체 필드로 정렬
    let mut people = vec![
        Person { name: String::from("Bob"), age: 30 },
        Person { name: String::from("Alice"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
    ];
    people.sort_by(|a, b| a.age.cmp(&b.age));
    for p in &people {
        println!("{}: {}세", p.name, p.age);
    }
}

struct Person {
    name: String,
    age: u32,
}

// 클로저를 매개변수로 받는 함수
fn apply_to_list<F>(list: &[i32], f: F)
where
    F: Fn(&i32), // Fn 트레이트 바운드
{
    println!("리스트 처리:");
    for item in list {
        f(item);
    }
}
