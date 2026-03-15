# 16. Concurrency

**"겁 없는 동시성(Fearless Concurrency)"** — 소유권 시스템과 타입 검사로 **컴파일 타임에** 데이터 레이스와 동시성 버그를 포착

## 핵심 개념

### 스레드 (threads.rs)
```rust
let handle = thread::spawn(move || {
    println!("스레드에서 실행");
});
handle.join().unwrap();  // 스레드 완료 대기, Result<T, E> 반환
```
- `move` 클로저: 소유권을 스레드로 이동 (스레드가 변수 캡처 시 필수)
- `.join()`: `Result` 반환 — 스레드가 panic했으면 `Err`

### 메시지 전달 (channels.rs)
**"메모리를 공유해서 통신하지 말고, 통신해서 메모리를 공유하라"** (Go 격언, Rust도 채택)
```rust
let (tx, rx) = mpsc::channel();  // 다중 생산자, 단일 소비자
let tx2 = tx.clone();            // 다중 생산자 생성

thread::spawn(move || tx.send(String::from("hi")).unwrap());
let received = rx.recv().unwrap();    // 블로킹 수신
// rx.try_recv()                       // 논블로킹 수신 → Result
for msg in rx { }                     // 채널이 닫힐 때까지 반복
```

### 공유 상태 (shared_state.rs)
```rust
let counter = Arc::new(Mutex::new(0));  // Arc: 멀티스레드용 Rc
let c = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = c.lock().unwrap();    // 락 획득 → MutexGuard
    *num += 1;
    // MutexGuard가 스코프 벗어나면 자동으로 락 해제 (Drop 트레이트)
});
```
- `Arc<Mutex<T>>`: 멀티스레드에서 안전한 공유 가변 상태 표준 패턴
- **데드락(Deadlock)**: 두 스레드가 서로가 가진 락을 기다릴 때 발생
  - 방지: 항상 같은 순서로 락 획득, 락 보유 시간 최소화
- **`RwLock<T>`**: 읽기 다중/쓰기 단독 (`Mutex` 대안 — 읽기 빈도가 높을 때 유리)

### Rust의 동시성 보장
- `Send` 트레이트: 스레드 간 소유권 전달 가능 — `Rc<T>`는 `Send` 아님
- `Sync` 트레이트: 여러 스레드에서 참조 가능 — `RefCell<T>`는 `Sync` 아님
- 대부분의 타입은 자동 구현 — 수동 구현 시 `unsafe` 필요

## 실행

```bash
rustc threads.rs && ./threads
rustc channels.rs && ./channels
rustc shared_state.rs && ./shared_state
```
