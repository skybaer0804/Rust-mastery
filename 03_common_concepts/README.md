# 03. Common Concepts

## 핵심 개념

### 변수와 가변성 (variables.rs)
- `let`: 불변 변수 (기본값)
- `let mut`: 가변 변수
- `const`: 상수 (타입 명시 필수, 컴파일 타임 결정, 모든 스코프에서 유효)
  ```rust
  const MAX_POINTS: u32 = 100_000;
  ```
- **Shadowing**: 같은 이름으로 재선언 가능 (타입 변경도 가능)
  ```rust
  let x = 5;
  let x = x + 1;       // 새 변수 x = 6
  let x = "hello";     // 타입도 변경 가능 (mut로는 불가)
  ```

### 주석 (Comments)
```rust
// 단행 주석
// 여러 줄은 각 줄마다 // 사용
```
- Rust는 `//` 단행 주석만 기본 사용 (블록 주석 `/* */`도 지원)
- 문서화 주석은 `///` (cargo doc으로 HTML 문서 생성)

### 데이터 타입 (data_types.rs)
- **정수형**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` / `u8` ~ `usize`
  - **정수 오버플로우**:
    - debug 빌드: panic 발생
    - release 빌드: 2의 보수 래핑(wrapping) 동작
- **부동소수점**: `f32`, `f64` (기본 `f64`)
- **불리언**: `bool`
- **문자**: `char` (4바이트, 유니코드 스칼라 값 — 이모지, 한글 등 모두 표현 가능)
- **튜플**: `(i32, f64, char)` — 고정 길이, 구조분해 가능
  ```rust
  let tup = (500, 6.4, 'z');
  let (x, y, z) = tup;  // 구조분해
  let first = tup.0;    // 인덱스 접근
  ```
- **배열**: `[i32; 5]` — 고정 길이, 스택 할당
  - 범위 초과 접근 시 **런타임 에러(panic)** 발생 (다른 언어와 달리 메모리 안전 보장)

### 함수 (functions.rs)
- `fn` 키워드로 정의
- 매개변수 타입 명시 필수
- 반환 타입은 `->` 뒤에 명시
- 마지막 **표현식(expression)** 이 암묵적 반환값 (세미콜론 없이)
  ```rust
  fn add(x: i32, y: i32) -> i32 {
      x + y  // 세미콜론 없음 → 반환값
  }
  ```
- **구문(statement)** vs **표현식(expression)**: 구문은 값을 반환하지 않음

### 제어 흐름 (control_flow.rs)
- `if` / `else if` / `else`: 조건문 (조건은 반드시 `bool` — 자동 형변환 없음)
- `let` 구문에서 `if` 사용 가능 (삼항 연산자 대신)
  ```rust
  let number = if condition { 5 } else { 6 };
  ```
- `loop`: 무한 루프 — `break`로 값 반환 가능
  ```rust
  let result = loop {
      counter += 1;
      if counter == 10 { break counter * 2; }  // loop에서 값 반환
  };
  ```
- `while`: 조건부 반복
- `for x in collection`: 이터레이터 순회 (가장 안전하고 권장)
  ```rust
  for x in (1..4).rev() { }  // .rev()로 역순 순회
  ```

## 실행

```bash
rustc variables.rs && ./variables
rustc data_types.rs && ./data_types
rustc functions.rs && ./functions
rustc control_flow.rs && ./control_flow
```
