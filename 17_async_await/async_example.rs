// Async/Await — 비동기 프로그래밍 기본 개념
// 참고: 실제 async/await는 tokio 등 런타임이 필요합니다.
// 이 예제는 개념 설명을 위한 동기적 시뮬레이션입니다.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// 커스텀 Future 구현 (학습용)
struct ReadyFuture {
    value: i32,
}

impl Future for ReadyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 즉시 준비됨
        Poll::Ready(self.value)
    }
}

// async fn은 Future를 반환하는 함수의 문법적 설탕
// async fn hello() -> String { ... }
// 위는 다음과 동일:
// fn hello() -> impl Future<Output = String> { ... }

fn main() {
    println!("=== Async/Await 개념 ===\n");

    // Future 트레이트 설명
    println!("Future 트레이트:");
    println!("  trait Future {{");
    println!("      type Output;");
    println!("      fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;");
    println!("  }}\n");

    // Poll 타입 설명
    println!("Poll 타입:");
    println!("  Poll::Ready(value) — 완료됨");
    println!("  Poll::Pending      — 아직 진행 중\n");

    // async/await 기본 문법
    println!("async/await 문법:");
    println!("  async fn fetch_data() -> String {{");
    println!("      let response = make_request().await;  // Future 완료 대기");
    println!("      response");
    println!("  }}\n");

    // 주요 개념 정리
    println!("주요 개념:");
    println!("  1. async fn → Future를 반환");
    println!("  2. .await → Future 완료까지 대기 (스레드 차단 아님!)");
    println!("  3. 런타임(tokio, async-std)이 Future를 실행");
    println!("  4. Pin — self-referential struct의 메모리 위치 고정");
    println!();

    // 동시 실행 패턴
    println!("동시 실행 패턴:");
    println!("  join!(future1, future2)  — 모든 Future 동시 실행, 모두 완료 대기");
    println!("  select!(future1, future2) — 먼저 완료되는 것 선택");
    println!("  tokio::spawn(future)      — 별도 태스크로 실행");
    println!();

    // 실제 사용 예시 (의사 코드)
    println!("실제 사용 예시 (tokio):");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let body = reqwest::get(url).await?.text().await?;");
    println!("      println!(\"{{body}}\");");
    println!("  }}");

    println!("\n(실제 async 실행은 tokio 등 런타임 의존성이 필요합니다)");
}
