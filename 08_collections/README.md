# 08. Collections

## 핵심 개념

### Vec<T> (vectors.rs)
- `Vec::new()` 또는 `vec![]` 매크로로 생성
- `.push()`: 요소 추가
- 인덱스(`&v[i]`) 또는 `.get(i)` → `Option<&T>`로 접근
- `for x in &v`: 순회 (불변)
- `for x in &mut v`: 순회 (가변)
- enum으로 서로 다른 타입 저장 가능

### String (strings.rs)
- `String`: 힙 할당, 가변, 소유됨
- `&str`: 문자열 슬라이스 (불변 참조)
- `String::from()`, `.to_string()`: 생성
- `+` 연산자 또는 `format!` 매크로로 결합
- UTF-8 인코딩: 인덱싱 불가, `.chars()` 사용

### HashMap<K, V> (hashmaps.rs)
- `HashMap::new()`로 생성
- `.insert()`: 삽입 (같은 키면 덮어쓰기)
- `.entry().or_insert()`: 없을 때만 삽입
- `.get()` → `Option<&V>`로 접근
- `for (k, v) in &map`: 순회

## 실행

```bash
rustc vectors.rs && ./vectors
rustc strings.rs && ./strings
rustc hashmaps.rs && ./hashmaps
```
