// Hello, World! — Rust의 첫 번째 프로그램
// println!은 매크로이다 (함수가 아님 — !에 주목)

fn main() {
    println!("Hello, World!");
    println!("안녕하세요, Rust!");

    // 포맷 문자열 사용
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // 여러 값 출력
    let x = 5;
    let y = 10;
    println!("{} + {} = {}", x, y, x + y);
}
