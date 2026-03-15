# 20. Advanced

## 핵심 개념

### Unsafe Rust (unsafe_example.rs)
빌림 검사기 비활성화가 아님 — **특정 5가지 작업만 허용**하는 별도 모드:
1. 원시 포인터(`*const T`, `*mut T`) 역참조
2. unsafe 함수/메서드 호출
3. 가변 정적 변수(`static mut`) 접근/수정
4. unsafe 트레이트 구현
5. `union` 필드 접근

```rust
unsafe {
    let raw = &x as *const i32;
    println!("{}", *raw);  // 원시 포인터 역참조
}

unsafe fn dangerous() { }
unsafe { dangerous(); }  // unsafe 함수 호출
```

#### FFI (Foreign Function Interface)
```rust
extern "C" {
    fn abs(input: i32) -> i32;  // C 표준 라이브러리 함수 선언
}

fn main() {
    unsafe { println!("{}", abs(-3)); }
}
```
- `extern "C"`: C ABI로 외부 함수 호출
- Rust 함수를 C에서 호출: `#[no_mangle] pub extern "C" fn call_from_c() { }`

#### 가변 정적 변수
```rust
static mut COUNTER: u32 = 0;
// 읽기/쓰기 모두 unsafe — 데이터 레이스 위험
unsafe { COUNTER += 1; }
```

### 고급 트레이트
#### 연관 타입
```rust
trait Iterator {
    type Item;  // 구현 시 구체 타입 지정 (제네릭 매개변수보다 간결)
    fn next(&mut self) -> Option<Self::Item>;
}
```

#### 연산자 오버로딩 (std::ops)
```rust
use std::ops::Add;
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
```

#### 완전 정규화 문법
```rust
// 같은 이름의 메서드가 여러 트레이트에 있을 때 명확히 지정
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

#### 뉴타입 패턴 (Newtype Pattern)
```rust
struct Wrapper(Vec<String>);  // 고아 규칙 우회: 외부 타입에 외부 트레이트 구현
impl fmt::Display for Wrapper { ... }
```

#### 슈퍼트레이트
```rust
trait OutlinePrint: fmt::Display { }  // Display 구현이 전제되어야 함
```

### 고급 타입
#### 타입 별칭
```rust
type Kilometers = i32;             // 별칭 (별개 타입 아님)
type Thunk = Box<dyn Fn() -> String>;  // 긴 타입 단순화
```

#### 결코 반환하지 않는 타입 `!`
```rust
fn diverges() -> ! { panic!("never returns"); }
// loop, continue, panic! 등이 ! 타입
```

#### 동적 크기 타입(DST)
```rust
// str, [T]: 크기를 컴파일 타임에 알 수 없음 → &str, &[T] (포인터+길이)로 사용
// dyn Trait: 동적 크기 → Box<dyn Trait>로 사용
```

### 고급 함수와 클로저
```rust
fn apply(f: fn(i32) -> i32, x: i32) -> i32 { f(x) }  // 함수 포인터
// fn 타입: Fn, FnMut, FnOnce 모두 구현 — 클로저 대신 함수 포인터 전달 가능
```

### 매크로 (macros.rs)
#### 선언적 매크로 (macro_rules!)
```rust
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        { let mut v = Vec::new(); $( v.push($x); )* v }
    };
}
```
- 패턴 매칭 기반 코드 생성
- 컴파일 전 코드 변환

#### 절차적 매크로 (3종)
| 종류 | 예시 | 설명 |
|---|---|---|
| derive | `#[derive(Debug)]` | 구조체/enum에 트레이트 자동 구현 |
| attribute-like | `#[route(GET, "/")]` | 함수/구조체에 커스텀 어트리뷰트 |
| function-like | `sql!(SELECT * FROM t)` | 함수 호출처럼 보이는 매크로 |
- `proc-macro` 크레이트로 구현, `TokenStream` 입출력

## 실행

```bash
rustc unsafe_example.rs && ./unsafe_example
rustc macros.rs && ./macros
```
