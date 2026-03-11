// Box<T> — 힙 할당 스마트 포인터

// 재귀적 타입 정의 — Box 없이는 컴파일 에러 (크기를 알 수 없음)
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

// Deref 트레이트 구현 예시
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// Drop 트레이트 구현 예시
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer `{}` drop!", self.data);
    }
}

fn main() {
    // === 기본 Box 사용 ===
    let b = Box::new(5);
    println!("b = {}", b);

    // === 재귀적 타입 (cons list) ===
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("cons list: {:?}", list);

    // === Deref — 역참조 ===
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // Box는 Deref 구현

    // 커스텀 MyBox
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y); // *y는 *(y.deref())와 같음

    // 역참조 강제 변환 (Deref coercion)
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> → &String → &str (자동 변환)

    // === Drop — 자동 정리 ===
    let _c = CustomSmartPointer {
        data: String::from("first"),
    };
    let _d = CustomSmartPointer {
        data: String::from("second"),
    };
    println!("CustomSmartPointers 생성됨");

    // 조기 해제 — std::mem::drop
    let e = CustomSmartPointer {
        data: String::from("early drop"),
    };
    println!("조기 해제 전");
    drop(e); // std::mem::drop으로 즉시 해제
    println!("조기 해제 후");
    // e.drop(); // 직접 호출 불가! (double free 방지)

    println!("main 종료 — 나머지 자동 drop");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
