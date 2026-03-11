// 변수와 가변성

fn main() {
    // 불변 변수 (기본)
    let x = 5;
    println!("x = {}", x);
    // x = 6; // 컴파일 에러! 불변 변수는 재대입 불가

    // 가변 변수
    let mut y = 5;
    println!("y = {}", y);
    y = 6; // OK — mut로 선언했으므로
    println!("y = {}", y);

    // 상수 — 타입 명시 필수, 컴파일 타임에 결정
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    // Shadowing — 같은 이름으로 재선언 가능
    let z = 5;
    let z = z + 1; // 새로운 변수 z (이전 z를 가림)
    let z = z * 2;
    println!("z = {}", z); // 12

    // Shadowing으로 타입 변경 가능
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize (타입이 바뀜!)
    println!("spaces = {}", spaces); // 3
}
