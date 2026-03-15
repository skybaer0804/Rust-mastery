# 06. Enums and Pattern Matching

## 핵심 개념

### Enum (enums.rs)
- `enum` 키워드로 열거형 정의
- 각 **배리언트(Variant)** 에 서로 다른 타입과 양의 데이터 포함 가능
  ```rust
  enum Message {
      Quit,                       // 데이터 없음
      Move { x: i32, y: i32 },   // 구조체처럼 이름 있는 필드
      Write(String),              // String 하나
      ChangeColor(i32, i32, i32), // 튜플처럼 여러 값
  }
  ```
- `Option<T>`: `Some(T)` 또는 `None` — **null 대신 사용**, 타입 시스템으로 null 안전성 보장
  ```rust
  let some_number: Option<i32> = Some(5);
  let no_value: Option<i32> = None;
  ```
- `impl`로 메서드 정의 가능

### 패턴 매칭 (pattern_matching.rs)
- `match`: 모든 가능한 경우를 처리 (**exhaustive** — 빠진 케이스 있으면 컴파일 에러)
- 각 arm은 `패턴 => 표현식` 형태

#### 값 바인딩
```rust
match coin {
    Coin::Quarter(state) => println!("State: {:?}", state),  // 배리언트 내부 값 바인딩
    _ => (),
}
```

#### 포괄 패턴(catch-all)
```rust
match number {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    other => println!("other: {}", other),  // 나머지 값을 other로 바인딩
    // _ => ()  // 값을 무시할 때는 _
}
```

#### Option<T>와 match
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

- `_`: 나머지 모든 경우 (와일드카드, 값 무시)
- `if let`: 하나의 패턴만 관심 있을 때 간결한 문법
  ```rust
  if let Some(value) = config_max {
      println!("{}", value);
  } else {
      // else 절도 가능
  }
  ```
- `while let`: 패턴이 매칭되는 동안 반복
  ```rust
  while let Some(top) = stack.pop() {
      println!("{}", top);
  }
  ```

## 실행

```bash
rustc enums.rs && ./enums
rustc pattern_matching.rs && ./pattern_matching
```
