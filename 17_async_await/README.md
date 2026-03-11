# 17. Async/Await

## 핵심 개념

### 비동기 프로그래밍
- `async fn`: 비동기 함수 (Future 반환)
- `.await`: Future 완료 대기
- `Future` 트레이트: 비동기 연산의 추상화

### 주요 개념
- **런타임**: tokio, async-std 등 (Rust 표준 라이브러리에 미포함)
- **태스크**: 경량 비동기 실행 단위
- **핀(Pin)**: 메모리 위치 고정 (자기 참조 구조체)
- **Stream**: 비동기 이터레이터

### 동시 실행
- `join!`: 여러 Future 동시 실행
- `select!`: 먼저 완료되는 Future 선택

> 이 예제는 런타임 없이 기본 개념만 설명합니다.

## 실행

```bash
rustc async_example.rs && ./async_example
```
