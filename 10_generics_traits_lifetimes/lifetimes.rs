// 라이프타임 (Lifetimes)

// 라이프타임 어노테이션이 필요한 함수
// 반환값의 수명이 x와 y 중 짧은 것과 같음을 명시
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 구조체에 라이프타임 — 참조를 필드로 가질 때
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 라이프타임 생략 규칙 적용 — 어노테이션 불필요
    fn level(&self) -> i32 {
        3
    }

    // 세 번째 생략 규칙: &self의 라이프타임이 반환값에 적용
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목: {}", announcement);
        self.part
    }
}

fn main() {
    // === 기본 라이프타임 ===
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("더 긴 문자열: {}", result);
    }
    // 아래 코드는 에러 — string2가 스코프를 벗어났으므로
    // println!("{}", result);

    // === 유효한 사용 ===
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = "xyz"; // 문자열 리터럴은 'static
        result = longest(string1.as_str(), string2);
    }
    println!("더 긴 문자열: {}", result); // OK — string1이 더 길고 여전히 유효

    // === 구조체의 라이프타임 ===
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.split('.').next().expect("'.'을 찾을 수 없음");
        first_sentence = ImportantExcerpt { part: i };
    }
    // novel이 여전히 유효하므로 first_sentence도 유효
    println!("발췌: {:?}", first_sentence);
    println!("레벨: {}", first_sentence.level());
    println!("{}", first_sentence.announce_and_return_part("중요 발표"));

    // === 'static 라이프타임 ===
    // 프로그램 전체 기간 동안 유효
    let s: &'static str = "나는 정적 라이프타임을 가집니다";
    println!("{}", s);

    // === 제네릭 + 트레이트 바운드 + 라이프타임 조합 ===
    let result = longest_with_announcement("hello", "world", "비교 중...");
    println!("announcement 결과: {}", result);
}

// 제네릭, 트레이트 바운드, 라이프타임을 모두 사용하는 함수
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: std::fmt::Display,
{
    println!("알림: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
