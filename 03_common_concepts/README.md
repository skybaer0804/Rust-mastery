# 03. Common Concepts

## 핵심 개념

### 변수와 가변성 (variables.rs)
- `let`: 불변 변수 (기본값)
- `let mut`: 가변 변수
- `const`: 상수 (타입 명시 필수, 컴파일 타임 결정)
- **Shadowing**: 같은 이름으로 재선언 가능 (타입 변경도 가능)

### 데이터 타입 (data_types.rs)
- **정수형**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` / `u8` ~ `usize`
- **부동소수점**: `f32`, `f64`
- **불리언**: `bool`
- **문자**: `char` (4바이트, 유니코드)
- **튜플**: `(i32, f64, char)` — 고정 길이, 구조분해 가능
- **배열**: `[i32; 5]` — 고정 길이, 스택 할당

### 함수 (functions.rs)
- `fn` 키워드로 정의
- 매개변수 타입 명시 필수
- 반환 타입은 `->` 뒤에 명시
- 마지막 표현식이 암묵적 반환값 (세미콜론 없이)

### 제어 흐름 (control_flow.rs)
- `if` / `else if` / `else`: 조건문 (조건은 반드시 `bool`)
- `let` 구문에서 `if` 사용 가능
- `loop`, `while`, `for`: 반복문
- `for x in collection`: 이터레이터 순회

## 실행

```bash
rustc variables.rs && ./variables
rustc data_types.rs && ./data_types
rustc functions.rs && ./functions
rustc control_flow.rs && ./control_flow
```
