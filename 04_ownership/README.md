# 04. Ownership

## 핵심 개념

### 소유권이 필요한 이유
- Rust는 **가비지 컬렉터 없이** 메모리 안전성을 보장하는 언어
- 소유권 시스템으로 컴파일 타임에 메모리 관리 — 런타임 비용 없음

### 스택 vs 힙
- **스택**: 고정 크기 데이터 (정수, bool, 튜플 등) — LIFO, 빠름
- **힙**: 가변 크기 데이터 (String, Vec 등) — 메모리 할당/해제 비용 있음
- Copy 타입(스택)과 Move 타입(힙)의 동작 차이의 근본 원인

### 소유권 규칙 (ownership.rs)
1. Rust의 각 값은 **소유자(owner)** 변수를 가진다
2. 한 번에 하나의 소유자만 존재
3. 소유자가 스코프를 벗어나면 값이 **drop** 된다

- **이동(Move)**: `String` 등 힙 데이터는 대입 시 소유권 이동 (원본 무효화)
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;  // s1은 무효, s2만 유효
  ```
- **복사(Copy)**: 정수, bool 등 스택 데이터는 `Copy` 트레이트로 자동 복사
- **클론(Clone)**: `.clone()`으로 힙 데이터 깊은 복사 (명시적, 비용 있음)
- 함수 인자 전달/반환 시에도 소유권 이동 발생
  ```rust
  fn takes_ownership(s: String) { }     // s의 소유권 이동
  fn gives_ownership() -> String {       // 반환으로 소유권 전달
      String::from("hello")
  }
  ```

### `drop`과 RAII
- 스코프를 벗어날 때 Rust가 자동으로 `drop` 함수 호출
- **RAII(Resource Acquisition Is Initialization)** 패턴: 자원 획득과 해제를 스코프로 관리
- `std::mem::drop(value)`으로 조기 해제 가능

### 참조와 빌림 (references.rs)
- `&`: 불변 참조 (여러 개 동시 가능) — 소유권 없이 값을 빌려옴
- `&mut`: 가변 참조 (한 번에 하나만)
- **규칙**: 불변 참조와 가변 참조 동시 불가
  ```rust
  let mut s = String::from("hello");
  let r1 = &s;
  let r2 = &s;      // OK: 불변 참조 여러 개
  // let r3 = &mut s; // 에러: 불변 참조가 있는 동안 가변 참조 불가
  ```
- **NLL(Non-Lexical Lifetimes)**: 참조의 유효 범위는 마지막 사용 시점까지
  ```rust
  let r1 = &s;
  println!("{}", r1); // r1의 마지막 사용
  let r3 = &mut s;    // OK: r1이 더 이상 사용되지 않으므로
  ```
- **댕글링 참조(Dangling Reference)**: 이미 해제된 메모리를 가리키는 참조 → 컴파일 에러
  ```rust
  // fn dangle() -> &String {  // 에러: 반환 후 s가 drop되어 댕글링
  //     let s = String::from("hello");
  //     &s
  // }
  ```

### 슬라이스 (slices.rs)
- `&str`: 문자열 슬라이스 — **문자열 리터럴의 타입도 `&str`** (바이너리에 저장된 슬라이스)
  ```rust
  let s = String::from("hello world");
  let hello = &s[0..5];  // &str 슬라이스
  let literal = "hello"; // 타입: &str
  ```
- `&[i32]`: 배열 슬라이스
- 범위 문법: `&s[0..5]`, `&s[..5]`, `&s[2..]`, `&s[..]`
- 소유권 없이 컬렉션의 일부를 참조

## 실행

```bash
rustc ownership.rs && ./ownership
rustc references.rs && ./references
rustc slices.rs && ./slices
```
