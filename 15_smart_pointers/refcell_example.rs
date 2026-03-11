// RefCell<T> — 내부 가변성 (Interior Mutability)

use std::cell::RefCell;
use std::rc::Rc;

// 내부 가변성 패턴: 불변 참조를 통해 값을 변경
#[derive(Debug)]
struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }

    fn send(&self, message: &str) {
        // &self (불변)이지만 RefCell 덕분에 내부 값 변경 가능
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

// Rc<RefCell<T>> — 여러 소유자 + 가변성
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // === RefCell 기본 사용 ===
    let data = RefCell::new(5);

    // borrow: 불변 참조 (여러 개 가능)
    {
        let r1 = data.borrow();
        let r2 = data.borrow();
        println!("불변 참조: {} {}", r1, r2);
    } // r1, r2 해제

    // borrow_mut: 가변 참조 (하나만 가능)
    {
        let mut r3 = data.borrow_mut();
        *r3 += 10;
    } // r3 해제

    println!("변경 후: {:?}", data);

    // === 규칙 위반 시 런타임 panic ===
    // let r1 = data.borrow();
    // let r2 = data.borrow_mut(); // panic! 불변 참조와 가변 참조 동시 불가

    // === MockMessenger 예시 ===
    let messenger = MockMessenger::new();
    messenger.send("첫 번째 메시지");
    messenger.send("두 번째 메시지");
    println!("보낸 메시지: {:?}", messenger.sent_messages.borrow());
    println!("메시지 수: {}", messenger.sent_messages.borrow().len());

    // === Rc<RefCell<T>> — 여러 소유자 + 가변 ===
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("변경 전:");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    // 공유된 값 변경 — 모든 리스트에 반영됨!
    *value.borrow_mut() += 10;

    println!("변경 후 (value += 10):");
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
