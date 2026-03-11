// 채널 (Channels) — 메시지 전달 동시성

use std::sync::mpsc; // multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    // === 기본 채널 ===
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("안녕하세요");
        tx.send(val).unwrap();
        // println!("{}", val); // 에러! 소유권이 send로 이동됨
    });

    let received = rx.recv().unwrap(); // 블로킹 수신
    println!("받은 메시지: {}", received);

    println!("--- 기본 채널 완료 ---\n");

    // === 여러 메시지 전송 ===
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // rx를 이터레이터로 사용 — 채널 닫힐 때까지 수신
    for received in rx {
        println!("받음: {}", received);
    }

    println!("--- 여러 메시지 완료 ---\n");

    // === 다중 생산자 (tx.clone()) ===
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 생산자 복제

    thread::spawn(move || {
        let vals = vec![
            String::from("tx1: 안녕"),
            String::from("tx1: 세계"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx: hello"),
            String::from("tx: world"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    for received in rx {
        println!("받음: {}", received);
    }

    println!("--- 다중 생산자 완료 ---");
}
