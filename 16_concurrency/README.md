# 16. Concurrency

## 핵심 개념

### 스레드 (threads.rs)
- `thread::spawn()`: 새 스레드 생성
- `.join()`: 스레드 완료 대기
- `move` 클로저: 소유권을 스레드로 이동

### 메시지 전달 (channels.rs)
- `mpsc::channel()`: 다중 생산자, 단일 소비자 채널
- `tx.send()`: 메시지 전송
- `rx.recv()`: 블로킹 수신
- `rx.try_recv()`: 논블로킹 수신
- `tx.clone()`: 다중 생산자 생성

### 공유 상태 (shared_state.rs)
- `Mutex<T>`: 상호 배제 — 한 번에 하나의 스레드만 접근
- `.lock()`: 락 획득 → `MutexGuard` 반환
- `Arc<T>`: 원자적 참조 카운팅 (멀티스레드용 `Rc`)
- `Arc<Mutex<T>>`: 멀티스레드에서 안전한 공유 가변 상태

### Rust의 동시성 보장
- `Send` 트레이트: 스레드 간 소유권 전달 가능
- `Sync` 트레이트: 여러 스레드에서 참조 가능

## 실행

```bash
rustc threads.rs && ./threads
rustc channels.rs && ./channels
rustc shared_state.rs && ./shared_state
```
