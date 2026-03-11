// 숫자 맞추기 게임 (간소화 버전 — rand 크레이트 없이)
// 핵심: 사용자 입력, 문자열 → 숫자 변환, match, loop

use std::io;

fn main() {
    println!("=== 숫자 맞추기 게임 ===");

    // rand 없이 간단한 "비밀 숫자" 사용
    let secret_number: u32 = 42;

    println!("1~100 사이의 숫자를 맞춰보세요!");

    loop {
        println!("숫자를 입력하세요:");

        // 가변 문자열 생성
        let mut guess = String::new();

        // 사용자 입력 읽기
        io::stdin()
            .read_line(&mut guess)
            .expect("입력을 읽지 못했습니다");

        // 문자열을 숫자로 변환 (에러 처리 포함)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력해주세요!");
                continue;
            }
        };

        println!("입력한 숫자: {}", guess);

        // 비교 (match와 Ordering 사용)
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("너무 작습니다!"),
            std::cmp::Ordering::Greater => println!("너무 큽니다!"),
            std::cmp::Ordering::Equal => {
                println!("정답입니다!");
                break;
            }
        }
    }
}
