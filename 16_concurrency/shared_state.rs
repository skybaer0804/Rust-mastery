// 공유 상태 동시성 — Mutex와 Arc

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // === Mutex 기본 사용 ===
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // 락 획득 → MutexGuard 반환
        *num = 6;
        // MutexGuard가 스코프를 벗어나면 자동으로 락 해제
    }

    println!("m = {:?}", m);

    // === 멀티스레드에서 Mutex 공유 ===
    // Arc (Atomic Reference Counting) — 멀티스레드용 Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("최종 카운터: {}", *counter.lock().unwrap()); // 10

    // === 실용적 예시: 공유 데이터 수집 ===
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let square = i * i;
            results.lock().unwrap().push((i, square));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    println!("\n제곱 결과:");
    for (num, square) in results.iter() {
        println!("  {} ^ 2 = {}", num, square);
    }

    // === 주의사항 ===
    // - Mutex<T>는 Send + Sync를 자동 구현
    // - 데드락 주의: 두 스레드가 서로의 락을 기다리면 영원히 대기
    // - Rc<T>는 Send가 아님 → 멀티스레드에서 사용 불가 → Arc<T> 사용
    println!("\n공유 상태 동시성 예제 완료");
}
