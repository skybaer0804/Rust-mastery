# 10. Generics, Traits, Lifetimes

## 핵심 개념

### 제네릭 (generics.rs)
구체 타입에 대한 추상화 — 코드 중복 제거
- 함수, 구조체, enum, 메서드에 타입 매개변수 사용
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
  struct Point<T> { x: T, y: T }
  ```
- `Option<T>`, `Vec<T>`, `HashMap<K, V>`, `Result<T, E>` 모두 제네릭 활용
- 컴파일 타임에 **단형성화(monomorphization)** → 각 구체 타입별 코드 생성 → 런타임 비용 없음

### 트레이트 (traits.rs)
공유 동작을 정의하는 인터페이스 (다른 언어의 interface와 유사)
- `trait Summary { fn summarize(&self) -> String; }`
- `impl Trait for Type`으로 구현
- **기본 구현**: 트레이트에 기본 메서드 제공 (오버라이드 가능)
- **고아 규칙(Orphan Rule)**: 외부 크레이트의 트레이트를 외부 타입에 구현 불가
  - `Display`를 `Vec<T>`에 구현 불가 — 둘 다 표준 라이브러리 소속

#### 트레이트 바운드
```rust
fn notify(item: &impl Summary) { ... }               // impl Trait 문법 (단순한 경우)
fn notify<T: Summary>(item: &T) { ... }              // 트레이트 바운드 문법 (복잡한 경우)
fn notify<T: Summary + Display>(item: &T) { ... }    // + 로 여러 트레이트 결합
fn notify<T>(item: &T) where T: Summary + Display { }  // where 절로 가독성 향상
```

#### 반환 타입
```rust
fn returns_summarizable() -> impl Summary { ... }  // 구체 타입 숨기기
// 단, 단일 구체 타입만 반환 가능 (여러 타입 반환 시 dyn 트레이트 사용)
```

#### 트레이트 객체 vs 제네릭
| | 제네릭 (`impl Trait` / `<T: Trait>`) | 트레이트 객체 (`dyn Trait`) |
|---|---|---|
| 디스패치 | 정적(컴파일 타임) | 동적(런타임, vtable) |
| 성능 | 빠름 | 약간 느림 |
| 이기종 컬렉션 | 불가 | 가능 |
| 반환 여러 타입 | 불가 | 가능 |

### 라이프타임 (lifetimes.rs)
참조의 유효 범위를 명시 — 댕글링 참조 방지
- `'a`: 라이프타임 어노테이션
  ```rust
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() { x } else { y }
  }
  ```
- **구조체의 라이프타임**: 참조를 필드로 가질 때 필요
  ```rust
  struct Important<'a> {
      part: &'a str,  // Important 인스턴스가 part 참조보다 오래 살 수 없음
  }
  ```

#### 라이프타임 생략 규칙 (컴파일러 자동 추론)
1. 각 입력 참조에 각각의 라이프타임 매개변수 부여
2. 입력 라이프타임이 하나뿐이면 출력 라이프타임도 동일하게 적용
3. 메서드의 경우 `&self`/`&mut self`가 있으면 출력 라이프타임 = self의 라이프타임

- **`'static`**: 프로그램 전체 기간 — 컴파일된 바이너리에 직접 저장된 문자열 리터럴
  ```rust
  let s: &'static str = "I have a static lifetime.";
  ```

## 실행

```bash
rustc generics.rs && ./generics
rustc traits.rs && ./traits
rustc lifetimes.rs && ./lifetimes
```
