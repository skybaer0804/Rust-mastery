// 멀티스레드 웹서버 (간소화 버전)

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// === 스레드 풀 구현 ===
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// 우아한 종료 (Graceful Shutdown)
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Worker {} 종료 중...", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {} 작업 수행 중", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} 연결 종료, 셧다운", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// === HTTP 요청 처리 ===
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    if bytes_read == 0 {
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = request.lines().next().unwrap_or("");
    println!("요청: {}", first_line);

    let (status_line, contents) = if first_line.starts_with("GET / ") {
        ("HTTP/1.1 200 OK", "<html><body><h1>안녕하세요!</h1><p>Rust 웹서버입니다.</p></body></html>")
    } else if first_line.starts_with("GET /hello ") {
        ("HTTP/1.1 200 OK", "<html><body><h1>Hello!</h1></body></html>")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "<html><body><h1>404 Not Found</h1></body></html>")
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let addr = "127.0.0.1:7878";

    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("서버 시작 실패: {} ({})", addr, e);
            eprintln!("포트가 이미 사용 중일 수 있습니다.");
            println!("\n=== 웹서버 코드 구조 설명 ===");
            println!("1. ThreadPool: 고정된 수의 워커 스레드");
            println!("2. Worker: 채널에서 Job을 수신하여 실행");
            println!("3. handle_connection: HTTP 요청 파싱 + 응답");
            println!("4. Drop: 우아한 종료 (모든 워커 join)");
            return;
        }
    };

    let pool = ThreadPool::new(4);

    println!("서버 시작: http://{}", addr);
    println!("Ctrl+C로 종료");

    // 데모용: 처음 5개 연결만 처리 후 종료
    for stream in listener.incoming().take(5) {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("연결 에러: {}", e);
            }
        }
    }

    println!("서버 종료 중...");
    // ThreadPool의 Drop이 호출되어 우아한 종료
}
