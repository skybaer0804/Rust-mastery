// 트레이트 (Traits)

use std::fmt;

// 트레이트 정의
trait Summary {
    fn summarize_author(&self) -> String;

    // 기본 구현
    fn summarize(&self) -> String {
        format!("({}님의 글...)", self.summarize_author())
    }
}

struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

// 트레이트 구현
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} — {}", self.title, self.author, &self.content[..20])
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize는 기본 구현 사용
}

// Display 트레이트 구현
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.content)
    }
}

// 트레이트 바운드 — 매개변수
fn notify(item: &impl Summary) {
    println!("속보! {}", item.summarize());
}

// 트레이트 바운드 문법 (동일)
fn notify_verbose<T: Summary>(item: &T) {
    println!("속보 (verbose)! {}", item.summarize());
}

// 여러 트레이트 바운드
fn notify_display(item: &(impl Summary + fmt::Display)) {
    println!("표시: {}", item); // Display
    println!("요약: {}", item.summarize()); // Summary
}

// where 절로 가독성 향상
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: Summary,
    U: fmt::Display,
{
    format!("{} / {}", t.summarize(), u)
}

// 트레이트를 반환 타입으로
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know"),
    }
}

fn main() {
    let article = NewsArticle {
        title: String::from("Penguins Win the Stanley Cup"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course as you probably already know, people"),
    };

    // 메서드 호출
    println!("기사 요약: {}", article.summarize());
    println!("트윗 요약: {}", tweet.summarize()); // 기본 구현

    // 트레이트 바운드 함수
    notify(&article);
    notify_verbose(&tweet);
    notify_display(&tweet);

    // where 절 함수
    let result = some_function(&article, &"hello world");
    println!("some_function: {}", result);

    // 반환 타입 트레이트
    let summary = returns_summarizable();
    println!("반환된 요약: {}", summary.summarize());
}
