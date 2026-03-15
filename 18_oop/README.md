# 18. OOP

Rust는 어떤 OOP 정의로는 맞고 다른 정의로는 아닐 수 있음 — OOP 패턴을 **구현할 수 있지만**, Rust 고유의 타입 시스템을 활용한 방식이 더 적합한 경우도 있음

## 핵심 개념

### Rust의 객체지향적 특성
- **캡슐화**: `pub` / private 접근 제어 — 구현 세부사항 숨기기
- **상속 대신 조합**: 트레이트로 동작 공유, 기본 구현으로 코드 재사용
  ```rust
  trait Animal {
      fn name(&self) -> &str;
      fn sound(&self) -> String { format!("{} says something", self.name()) }  // 기본 구현
  }
  ```
- **다형성**: 트레이트 객체 (`dyn Trait`) — 런타임 다형성

### 트레이트 객체
```rust
trait Draw { fn draw(&self); }

struct Screen {
    components: Vec<Box<dyn Draw>>,  // 이기종 타입을 하나의 Vec에 저장
}
```
- `Box<dyn Trait>`: 런타임에 실제 타입 결정 (동적 디스패치)
- vtable을 통한 메서드 호출

#### 정적 디스패치 vs 동적 디스패치
| | 정적 (`impl Trait` / 제네릭) | 동적 (`dyn Trait`) |
|---|---|---|
| 타입 결정 | 컴파일 타임 | 런타임 |
| 성능 | 빠름 (인라인 가능) | vtable 조회 비용 |
| 이기종 컬렉션 | 불가 | 가능 |
| 바이너리 크기 | 단형성화로 증가 | 단일 코드 경로 |

#### 객체 안전성(Object Safety) 조건
트레이트 객체(`dyn Trait`)로 사용 가능하려면:
- 반환 타입이 `Self`가 아닐 것
- 제네릭 타입 매개변수가 없을 것
```rust
// Clone은 객체 안전하지 않음: fn clone(&self) -> Self
// trait Draw: Clone { }  // 이 경우 dyn Draw 불가
```

### 상태 패턴
#### 트레이트 객체 방식 (OOP 스타일)
```rust
trait State { fn request_review(self: Box<Self>) -> Box<dyn State>; }
struct Post { state: Option<Box<dyn State>>, content: String }
```
- 런타임에 상태 전환
- 상태 추가 시 새 구조체 추가만 필요

#### Rust 방식 — 타입으로 상태 인코딩
```rust
struct Draft { content: String }
struct PendingReview { content: String }
struct Published { content: String }

impl Draft {
    fn request_review(self) -> PendingReview {
        PendingReview { content: self.content }
    }
}
```
- **컴파일 타임 검증**: 잘못된 상태에서 메서드 호출 시 컴파일 에러
- 트레이트 객체 방식보다 안전하고 성능도 좋음

## 실행

```bash
rustc oop_example.rs && ./oop_example
```
