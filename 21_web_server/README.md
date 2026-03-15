# 21. Web Server

소유권, 트레이트, 제네릭, 스마트 포인터, 동시성 등 **책 전체 개념을 종합**하는 최종 프로젝트.
단일 스레드 → 멀티스레드 웹서버로 점진적 개선.

## 핵심 개념

### TCP 서버
```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
}
```

### HTTP 파싱
```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{contents}",
        contents.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
}
```

### 스레드 풀 구조
```rust
type Job = Box<dyn FnOnce() + Send + 'static>;  // 타입 별칭

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
```

#### ThreadPool::new
```rust
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));  // 여러 워커가 공유
        let workers = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static  // 스레드 경계를 넘어 한 번 실행
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
```
- `Arc<Mutex<Receiver<Job>>>`: 여러 워커 스레드가 채널 수신단을 공유하는 이유
  - `Arc`: 여러 소유자, `Mutex`: 한 번에 한 워커만 작업 받기

#### Graceful Shutdown (Drop 구현)
```rust
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());  // 채널 닫기 → 워커들의 루프 종료
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {  // Option::take()로 소유권 이동
                thread.join().unwrap();
            }
        }
    }
}
```
- `Option::take()`: `Some(T)` → `None`으로 교체하며 `T` 소유권 획득
- 채널 닫힘 → 워커의 `recv()` 에러 → 루프 종료 → join 가능

### 개선 방향
- `async`/`await` + `tokio`로 리팩토링 시 스레드 풀 없이 더 효율적
- 연결 수 제한, 타임아웃, HTTP/2 지원 추가

## 실행

```bash
rustc web_server.rs && ./web_server
# Cargo 프로젝트로:
cargo run
# 브라우저에서 http://127.0.0.1:7878 접속
```
