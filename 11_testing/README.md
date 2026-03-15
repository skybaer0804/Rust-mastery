# 11. Testing

## 핵심 개념

### 테스트 작성
- `#[test]` 어트리뷰트로 테스트 함수 표시
- `assert!(condition)`: 조건이 true인지 확인
  - 커스텀 메시지 추가 가능: `assert!(cond, "값이 {}여야 함", value)`
- `assert_eq!(left, right)`: 값 비교 — 실패 시 두 값을 모두 출력
  ```
  thread 'test' panicked at 'assertion failed: `(left == right)`
    left: `2`,
   right: `3`'
  ```
- `assert_ne!(left, right)`: 같지 않음 확인
- `#[should_panic]`: panic이 발생해야 통과
  ```rust
  #[should_panic(expected = "between 1 and 100")]  // 특정 메시지 포함 여부 검증
  fn test_panic() { ... }
  ```
- `Result<(), String>` 반환으로 `?` 사용 가능 (`#[should_panic]`과 함께 사용 불가)

### 테스트 실행
- `cargo test`: 모든 테스트 병렬 실행
- `cargo test 함수명`: 특정 테스트만 실행 (부분 문자열 매칭)
- `cargo test -- --test-threads=1`: 단일 스레드로 실행 (파일 공유 등 격리 필요 시)
- `cargo test -- --show-output`: 성공한 테스트의 `println!` 출력도 표시
- `#[ignore]`: 기본 실행에서 제외 (오래 걸리는 테스트)
- `cargo test -- --ignored`: 무시된 테스트만 실행
- `cargo test -- --include-ignored`: 모든 테스트 실행

### 테스트 구조
#### 단위 테스트
```rust
#[cfg(test)]  // cargo test 시에만 컴파일
mod tests {
    use super::*;  // 부모 모듈의 항목 가져오기

    #[test]
    fn it_works() { ... }
}
```
- 같은 파일에 위치 → **비공개 함수도 테스트 가능** (자식 모듈이 부모에 접근 가능한 규칙)

#### 통합 테스트
- `tests/` 디렉토리에 별도 파일
- 라이브러리 크레이트만 가능 (바이너리 크레이트는 직접 통합 테스트 불가)
- 공통 헬퍼 함수: `tests/common/mod.rs` 또는 `tests/common.rs` 파일로 분리
  ```rust
  // tests/integration_test.rs
  mod common;  // tests/common.rs 가져오기
  common::setup();
  ```

## 실행

```bash
rustc --test testing_example.rs && ./testing_example
# 또는 Cargo 프로젝트에서:
cargo test
```
