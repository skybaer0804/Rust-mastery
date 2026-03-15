# 19. Patterns

타입의 구조에 대해 매칭하기 위한 특수 문법 — Rust 전반에서 사용

## 핵심 개념

### 패턴이 사용되는 곳
```rust
match value { pattern => expression }      // match arms
if let Some(x) = option { }               // if let
while let Some(top) = stack.pop() { }     // while let
for (index, value) in v.iter().enumerate() { }  // for
let (a, b, c) = (1, 2, 3);               // let 구조분해
fn foo(&(x, y): &(i32, i32)) { }         // 함수 매개변수 구조분해
```

### 반박 가능성
- **반박 불가능(irrefutable)**: 항상 매칭 → `let`, `for`, 함수 매개변수에 사용
  ```rust
  let x = 5;  // 항상 매칭
  ```
- **반박 가능(refutable)**: 매칭 실패 가능 → `if let`, `while let`, `match`에 사용
  ```rust
  if let Some(x) = value { }  // None이면 매칭 실패
  ```

### 패턴 문법

#### 리터럴, 변수, 범위
```rust
match x {
    1 => "one",
    2 | 3 => "two or three",    // | : or 패턴
    4..=9 => "four to nine",    // ..= : 범위
    _ => "other",               // _ : 와일드카드 (값 무시)
}
```

#### 구조분해
```rust
// 튜플
let (a, b, c) = (1, 2, 3);

// 구조체
struct Point { x: i32, y: i32 }
let Point { x, y } = p;              // 필드명 = 변수명
let Point { x: a, y: b } = p;       // 이름 변경

// 구조체 + 일부 무시
let Point { x, .. } = p;            // y는 무시

// enum
match msg {
    Message::Move { x, y } => { },
    Message::Write(text) => { },
    Message::Quit => { },
}
```

#### 나머지 무시
```rust
let (first, .., last) = (1, 2, 3, 4, 5);  // 중간 값 무시
struct Point3D { x: i32, y: i32, z: i32 }
let Point3D { x, .. } = p;                 // y, z 무시
```

#### 매치 가드 (match guard)
```rust
match num {
    n if n < 0 => "negative",    // 패턴 + 추가 조건
    0 => "zero",
    n if n % 2 == 0 => "positive even",
    _ => "positive odd",
}
```
- `|` 패턴 + 매치 가드: `4 | 5 | 6 if y` → `(4 | 5 | 6) if y`

#### @ 바인딩
```rust
match age {
    n @ 1..=12 => println!("어린이: {n}"),   // 범위 매칭 + 변수 바인딩 동시에
    n @ 13..=19 => println!("십대: {n}"),
    n => println!("기타: {n}"),
}
```

## 실행

```bash
rustc patterns.rs && ./patterns
```
