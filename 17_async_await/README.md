# 17. Async/Await

## 핵심 개념

### 왜 비동기인가?
- **I/O-bound** 작업(네트워크, 파일): 기다리는 동안 다른 작업 수행 → 비동기 적합
- **CPU-bound** 작업: 병렬 스레드가 더 적합
- **스레드 vs 비동기**: 스레드는 OS 자원 비용 있음, 비동기는 경량 태스크로 수천 개 운용 가능

### 기본 문법
```rust
async fn fetch_data() -> String {
    // async fn은 Future를 반환
    String::from("data")
}

async fn main_task() {
    let result = fetch_data().await;  // Future 완료까지 대기 (스레드 블로킹 아님, 실행 양보)
    println!("{}", result);
}
```
- `async fn`은 호출 시 즉시 실행되지 않고 `Future`를 반환
- `.await`는 Future가 완료될 때까지 현재 태스크를 일시 중단, 다른 태스크 실행
- **`async` 블록**: `async { ... }` — 표현식으로 Future 생성

### Future 트레이트
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
- `poll()`이 `Pending`을 반환하면 런타임에 제어권 반환, `Ready(value)`이면 완료
- 직접 구현할 일은 드물지만 동작 원리 이해 중요

### Pin<T>
- `async fn` 내부는 자기 참조 구조체가 될 수 있음 → 메모리 이동 시 포인터 무효화
- `Pin<T>`: 데이터를 메모리에 고정 → 이동 불가 보장
- 런타임과 `async`/`await` 사용 시 대부분 자동 처리됨

### 런타임 (별도 크레이트 필요)
Rust 표준 라이브러리는 런타임을 포함하지 않음 — 가장 많이 쓰이는 선택지:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```
```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {  // 태스크 생성
        "hello from task"
    });
    let result = handle.await.unwrap();
}
```

### 동시 실행
```rust
// join!: 모든 Future를 동시에 실행, 모두 완료될 때까지 대기
let (a, b) = tokio::join!(task_a(), task_b());

// select!: 가장 먼저 완료되는 Future 선택 (나머지는 취소)
tokio::select! {
    result = task_a() => println!("A 완료: {result}"),
    result = task_b() => println!("B 완료: {result}"),
}
```

### Stream
- `Iterator`의 비동기 버전 — 비동기적으로 값을 순차 생산
- `StreamExt` 트레이트로 `.next().await`, `.map()`, `.filter()` 등 사용

### async에서 에러 처리
```rust
async fn might_fail() -> Result<String, Box<dyn std::error::Error>> {
    let data = fetch().await?;  // ?로 에러 전파 가능
    Ok(data)
}
```

> 이 예제는 런타임 없이 기본 개념만 설명합니다.

## 실행

```bash
rustc async_example.rs && ./async_example
# tokio 사용 시:
cargo run
```
