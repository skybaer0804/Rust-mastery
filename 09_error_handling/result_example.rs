// Result<T, E> — 복구 가능한 에러

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // === match로 Result 처리 ===
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("파일 열기 성공");
            file
        }
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("파일을 찾을 수 없음 — 이것은 예상된 에러입니다");
                return;
            }
            other_error => {
                println!("파일 열기 에러: {:?}", other_error);
                return;
            }
        },
    };

    // === unwrap과 expect ===
    // unwrap: Ok면 값, Err면 panic
    // let f = File::open("hello.txt").unwrap();

    // expect: unwrap + 커스텀 메시지
    // let f = File::open("hello.txt").expect("파일을 열 수 없습니다");

    // === ? 연산자 사용 ===
    match read_username_from_file() {
        Ok(username) => println!("사용자명: {}", username),
        Err(e) => println!("에러: {}", e),
    }

    // 간결한 버전
    match read_username_short() {
        Ok(username) => println!("사용자명 (간결): {}", username),
        Err(e) => println!("에러 (간결): {}", e),
    }

    // 더 간결한 버전
    match read_username_shortest() {
        Ok(username) => println!("사용자명 (최단): {}", username),
        Err(e) => println!("에러 (최단): {}", e),
    }

    println!("Result 예제 완료");
}

// ? 연산자 — Err면 자동으로 현재 함수에서 반환
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// ? 연산자로 간결하게
fn read_username_short() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 체이닝으로 더 간결하게
fn read_username_shortest() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
