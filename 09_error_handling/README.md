# 09. Error Handling

## 핵심 개념

Rust는 **예외(exception) 메커니즘이 없음** — 에러를 두 가지로 분류:
- **복구 가능한 에러(recoverable)**: `Result<T, E>` — 파일 없음, 입력 오류 등
- **복구 불가능한 에러(unrecoverable)**: `panic!` — 버그, 불변 조건 위반 등

### panic! (panic_example.rs)
- 복구 불가능한 에러 발생 시 사용
- 기본 동작: **스택 되감기(unwinding)** — 각 스택 프레임 정리 후 종료
- `Cargo.toml`에서 즉시 종료(abort)로 변경 가능 (바이너리 크기 축소)
  ```toml
  [profile.release]
  panic = 'abort'
  ```
- `RUST_BACKTRACE=1 cargo run`으로 전체 백트레이스 확인
- 배열 범위 초과 접근 시 자동 panic

### Result<T, E> (result_example.rs)
- `Ok(T)`: 성공값
- `Err(E)`: 에러값
- `match`로 처리
  ```rust
  match File::open("hello.txt") {
      Ok(file) => file,
      Err(error) => match error.kind() {
          ErrorKind::NotFound => { /* 생성 시도 */ },
          other => panic!("{:?}", other),
      },
  }
  ```
- `.unwrap()`: Ok면 값 반환, Err면 panic
- `.expect("msg")`: unwrap + 커스텀 메시지 (프로덕션에서 권장)

### ? 연산자
- `Result`를 반환하는 함수 안에서 사용
- `Err`면 자동으로 현재 함수에서 반환
- 에러 타입 자동 변환 (`From` 트레이트)
- 체이닝 가능: `File::open(path)?.read_to_string(&mut s)?`
- **`main` 함수에서도 사용 가능**:
  ```rust
  fn main() -> Result<(), Box<dyn Error>> {
      let f = File::open("hello.txt")?;
      Ok(())
  }
  ```

### Box<dyn Error>
- 여러 에러 타입을 하나로 처리하는 **트레이트 객체**
- "어떤 에러든 가능"을 의미 — 라이브러리 구현 없이 빠르게 처리할 때 유용

### 에러 처리 가이드라인
| 상황 | 권장 방식 |
|---|---|
| 프로토타입/예제 코드 | `unwrap()`, `expect()` |
| 테스트 코드 | `unwrap()`, `expect()` |
| 라이브러리 | `Result` 반환으로 호출자에게 위임 |
| 로직상 절대 실패 불가 | `unwrap()` (이유를 주석으로 명시) |
| 잘못된 값이 들어오면 안 되는 경우 | `panic!` (계약 위반) |

- **커스텀 에러 타입**: `std::error::Error` 트레이트 구현으로 의미 있는 에러 표현
- 실무에서는 `anyhow`(애플리케이션), `thiserror`(라이브러리) 크레이트 자주 사용

## 실행

```bash
rustc panic_example.rs && ./panic_example
rustc result_example.rs && ./result_example
```
