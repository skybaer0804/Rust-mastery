# 15. Smart Pointers

스마트 포인터 = 포인터 + **추가 메타데이터와 기능**
- 참조자는 데이터를 빌려올 뿐, 스마트 포인터는 **데이터를 소유**
- `String`, `Vec<T>` 도 스마트 포인터의 일종
- 핵심 트레이트: `Deref`(역참조 동작), `Drop`(정리 코드)

## 핵심 개념

### Box<T> (box_example.rs)
- 힙에 데이터 할당 — 스택에는 포인터만 저장
- 사용 시점:
  - 컴파일 타임에 크기를 모를 때 (재귀 타입)
  - 큰 데이터 이동 없이 소유권 전달 (포인터만 복사)
  - `Box<dyn Trait>` 트레이트 객체
- **재귀적 타입** 정의에 필수 (cons list 등):
  ```rust
  enum List { Cons(i32, Box<List>), Nil }
  ```

### Deref와 역참조 강제 변환
```rust
let x = 5;
let y = Box::new(x);
assert_eq!(5, *y);  // Deref 트레이트로 * 연산자 사용
```
- **역참조 강제 변환(Deref Coercion)**: 컴파일러가 자동으로 타입 변환
  ```rust
  fn hello(name: &str) { }
  let s = String::from("Rust");
  hello(&s);       // &String → &str 자동 변환 (Deref Coercion)
  hello(&s[..]);   // 명시적 버전
  ```

### Drop
- 스코프 벗어날 때 자동으로 `drop` 메서드 호출
- `std::mem::drop(value)`: 스코프 전에 조기 해제 (직접 `.drop()` 호출 불가)

### Rc<T> (rc_example.rs)
- **참조 카운팅**: 여러 소유자 허용 (단일 스레드만)
- `Rc::clone(&rc)`: 참조 카운트 증가 — 깊은 복사 아님 (빠름)
- `Rc::strong_count()`: 현재 강한 참조 수
- 불변 참조만 제공 → 가변성 필요 시 `RefCell<T>` 조합

### RefCell<T> (refcell_example.rs)
- **내부 가변성(Interior Mutability)**: 불변 참조를 통해 값 변경 가능
- 빌림 규칙을 **런타임**에 검사 (컴파일 타임이 아님)
- `.borrow()` → `Ref<T>` (불변), `.borrow_mut()` → `RefMut<T>` (가변)
- 규칙 위반 시 런타임 panic

#### Rc<RefCell<T>> 조합 패턴
```rust
let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
let a = Rc::clone(&shared);
let b = Rc::clone(&shared);
a.borrow_mut().push(4);  // 여러 소유자가 있지만 내부 값을 변경 가능
```

### Weak<T> — 순환 참조 방지
- `Rc::downgrade(&rc)` → `Weak<T>` (약한 참조, 카운트에 미포함)
- `weak.upgrade()` → `Option<Rc<T>>` (이미 해제됐으면 `None`)
- **부모 → 자식**: `Rc<T>`, **자식 → 부모**: `Weak<T>` 패턴으로 순환 참조 방지
  ```rust
  struct Node {
      children: Vec<Rc<Node>>,
      parent: RefCell<Weak<Node>>,  // 부모는 Weak으로 참조
  }
  ```

### 순환 참조 주의
- `Rc<T>` + `RefCell<T>` 조합 시 순환 참조 → 메모리 누수 가능
- Rust의 소유권이 메모리 누수를 완전히 방지하지는 않음

## 실행

```bash
rustc box_example.rs && ./box_example
rustc rc_example.rs && ./rc_example
rustc refcell_example.rs && ./refcell_example
```
