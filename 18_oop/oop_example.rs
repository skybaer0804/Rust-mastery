// 객체지향 프로그래밍 (OOP in Rust)

// === 캡슐화 — pub / private ===
mod collection {
    pub struct AveragedCollection {
        list: Vec<i32>,     // 비공개
        average: f64,       // 비공개
    }

    impl AveragedCollection {
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        // 비공개 메서드 — 내부 구현
        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

// === 다형성 — 트레이트 객체 (dyn Trait) ===
trait Draw {
    fn draw(&self);
}

struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button [{}x{}]: {}",
            self.width, self.height, self.label
        );
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox [{}x{}]: {:?}",
            self.width, self.height, self.options
        );
    }
}

// 트레이트 객체를 사용하는 Screen
struct Screen {
    // Box<dyn Draw> — 런타임 디스패치 (vtable 사용)
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw(); // 동적 디스패치
        }
    }
}

// === 상태 패턴 — 타입 시스템 활용 ===
struct Post {
    content: String,
}

struct DraftPost {
    content: String,
}

struct PendingReviewPost {
    content: String,
}

impl DraftPost {
    fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 상태 전환 — 소유권 이동으로 이전 상태 사용 불가
    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

impl Post {
    fn content(&self) -> &str {
        &self.content
    }
}

fn main() {
    // === 캡슐화 ===
    println!("=== 캡슐화 ===");
    let mut avg = collection::AveragedCollection::new();
    avg.add(10);
    avg.add(20);
    avg.add(30);
    println!("평균: {}", avg.average());
    avg.remove();
    println!("제거 후 평균: {}", avg.average());

    // === 다형성 (트레이트 객체) ===
    println!("\n=== 다형성 ===");
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
        ],
    };
    screen.run();

    // === 상태 패턴 (타입 시스템 활용) ===
    println!("\n=== 상태 패턴 ===");
    let mut post = DraftPost::new();
    post.add_text("Rust의 OOP는 다릅니다");
    // post.content(); // 컴파일 에러! DraftPost에는 content() 없음

    let post = post.request_review();
    // post.add_text("추가"); // 컴파일 에러! PendingReviewPost에는 add_text 없음

    let post = post.approve();
    println!("게시글 내용: {}", post.content());
}
