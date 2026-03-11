# 06. Enums and Pattern Matching

## 핵심 개념

### Enum (enums.rs)
- `enum` 키워드로 열거형 정의
- 각 배리언트에 데이터 포함 가능 (다양한 타입)
- `Option<T>`: `Some(T)` 또는 `None` — null 대신 사용
- `impl`로 메서드 정의 가능

### 패턴 매칭 (pattern_matching.rs)
- `match`: 모든 가능한 경우를 처리 (exhaustive)
- 각 arm은 `패턴 => 표현식` 형태
- `_`: 나머지 모든 경우 (와일드카드)
- `if let`: 하나의 패턴만 관심 있을 때 간결한 문법
- `while let`: 패턴이 매칭되는 동안 반복

## 실행

```bash
rustc enums.rs && ./enums
rustc pattern_matching.rs && ./pattern_matching
```
