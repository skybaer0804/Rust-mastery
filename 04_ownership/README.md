# 04. Ownership

## 핵심 개념

### 소유권 규칙 (ownership.rs)
1. Rust의 각 값은 **소유자(owner)** 변수를 가진다
2. 한 번에 하나의 소유자만 존재
3. 소유자가 스코프를 벗어나면 값이 **drop** 된다

- **이동(Move)**: `String` 등 힙 데이터는 대입 시 소유권 이동
- **복사(Copy)**: 정수, bool 등 스택 데이터는 Copy 트레이트로 복사
- **클론(Clone)**: `.clone()`으로 힙 데이터 깊은 복사
- 함수 인자 전달/반환 시에도 소유권 이동 발생

### 참조와 빌림 (references.rs)
- `&`: 불변 참조 (여러 개 동시 가능)
- `&mut`: 가변 참조 (한 번에 하나만)
- **규칙**: 불변 참조와 가변 참조 동시 불가
- 참조는 항상 유효해야 함 (dangling reference 방지)

### 슬라이스 (slices.rs)
- `&str`: 문자열 슬라이스
- `&[i32]`: 배열 슬라이스
- 범위 문법: `&s[0..5]`, `&s[..5]`, `&s[2..]`
- 소유권 없이 컬렉션의 일부를 참조

## 실행

```bash
rustc ownership.rs && ./ownership
rustc references.rs && ./references
rustc slices.rs && ./slices
```
