// Rc<T> — 참조 카운팅 스마트 포인터

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // === 문제: 여러 리스트가 같은 부분을 공유 ===
    // Box를 쓰면 소유권 문제 발생 — Rc로 해결

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 생성 후 참조 카운트: {}", Rc::strong_count(&a));

    // Rc::clone — 참조 카운트 증가 (깊은 복사 아님!)
    let b = Cons(3, Rc::clone(&a));
    println!("b 생성 후 참조 카운트: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c 생성 후 참조 카운트: {}", Rc::strong_count(&a));
        println!("c: {:?}", c);
    } // c가 스코프를 벗어남 → 참조 카운트 감소

    println!("c 스코프 종료 후 참조 카운트: {}", Rc::strong_count(&a));

    println!("\na: {:?}", a);
    println!("b: {:?}", b);

    // === Rc는 불변 참조만 제공 ===
    // 가변이 필요하면 Rc<RefCell<T>> 패턴 사용

    // === 실용적 예시: 그래프에서 공유 노드 ===
    let shared_data = Rc::new(String::from("공유 데이터"));
    let ref1 = Rc::clone(&shared_data);
    let ref2 = Rc::clone(&shared_data);

    println!("\n공유 데이터: {}", shared_data);
    println!("ref1: {}", ref1);
    println!("ref2: {}", ref2);
    println!("최종 참조 카운트: {}", Rc::strong_count(&shared_data));
}
