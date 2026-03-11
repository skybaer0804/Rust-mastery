// 스레드 (Threads)

use std::thread;
use std::time::Duration;

fn main() {
    // === 기본 스레드 생성 ===
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("스레드에서: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("메인에서: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join — 스레드 완료 대기
    handle.join().unwrap();
    println!("--- 첫 번째 스레드 완료 ---\n");

    // === move 클로저 — 소유권을 스레드로 이동 ===
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // move 없으면 컴파일 에러: v의 수명이 보장되지 않음
        println!("벡터: {:?}", v);
    });

    // println!("{:?}", v); // 에러! 소유권이 스레드로 이동됨
    handle.join().unwrap();
    println!("--- 두 번째 스레드 완료 ---\n");

    // === 여러 스레드 생성 ===
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("스레드 {} 시작", i);
            thread::sleep(Duration::from_millis(10));
            println!("스레드 {} 종료", i);
        });
        handles.push(handle);
    }

    // 모든 스레드 완료 대기
    for handle in handles {
        handle.join().unwrap();
    }
    println!("--- 모든 스레드 완료 ---");
}
