# 05. Structs

## 핵심 개념

### 구조체 정의 (structs.rs)
- `struct` 키워드로 정의
- 필드에 이름과 타입 지정
- **필드 초기화 축약**: 변수명과 필드명이 같으면 축약 가능
- **구조체 업데이트 문법**: `..other_instance`로 나머지 필드 복사
- **튜플 구조체**: `struct Color(i32, i32, i32)`
- **유닛 구조체**: `struct AlwaysEqual;` (필드 없음)

### 메서드 (methods.rs)
- `impl` 블록 안에 정의
- 첫 번째 매개변수는 `&self`, `&mut self`, 또는 `self`
- **연관 함수**: `self` 없이 정의 (예: `String::from`)
  - `::` 문법으로 호출
- 여러 `impl` 블록 가능

## 실행

```bash
rustc structs.rs && ./structs
rustc methods.rs && ./methods
```
