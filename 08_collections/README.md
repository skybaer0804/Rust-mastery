# 08. Collections

컬렉션은 **힙 메모리에 저장** — 고정 크기인 배열/튜플과 달리 런타임에 동적 크기 변경 가능

## 핵심 개념

### Vec<T> (vectors.rs)
- `Vec::new()` 또는 `vec![]` 매크로로 생성
- `.push()`: 요소 추가
- **길이(len)** vs **용량(capacity)**: len은 실제 요소 수, capacity는 재할당 없이 저장 가능한 수
  - 용량 초과 시 새 메모리 할당 후 복사 발생
- 인덱스(`&v[i]`) — 범위 초과 시 panic / `.get(i)` → `Option<&T>` — 범위 초과 시 `None`
- `for x in &v`: 순회 (불변)
- `for x in &mut v`: 순회 (가변)
- enum으로 서로 다른 타입 저장 가능
  ```rust
  enum Cell { Int(i32), Float(f64), Text(String) }
  let row = vec![Cell::Int(3), Cell::Float(1.0), Cell::Text(String::from("hi"))];
  ```

### String (strings.rs)
- `String`: 힙 할당, 가변, 소유됨 — 내부적으로 `Vec<u8>`
- `&str`: 문자열 슬라이스 (불변 참조)
- `String::from()`, `.to_string()`: 생성
- `+` 연산자 또는 `format!` 매크로로 결합

#### String 인덱싱이 불가한 이유
- UTF-8은 **가변 길이 인코딩** — 문자마다 1~4바이트 사용
- 인덱스로 O(1) 접근 시 유효한 유니코드 문자 경계를 보장할 수 없음
- 대신 3가지 뷰 사용:
  ```rust
  for b in "안녕".bytes()  { }   // u8 바이트 단위
  for c in "안녕".chars()  { }   // char(유니코드 스칼라 값) 단위
  // grapheme cluster(사용자 인식 문자)는 외부 크레이트 필요
  ```
- 범위 슬라이싱은 가능하나 바이트 경계가 어긋나면 panic
  ```rust
  let s = &"안녕"[0..3];  // OK: '안'은 3바이트
  // let s = &"안녕"[0..1];  // panic: 문자 경계 아님
  ```

### HashMap<K, V> (hashmaps.rs)
- `HashMap::new()`로 생성
- `.insert()`: 삽입 (같은 키면 덮어쓰기)
  - **소유권**: 키/값 삽입 시 `String` 같은 소유 타입은 소유권이 HashMap으로 이동
- `.entry().or_insert()`: 없을 때만 삽입
  ```rust
  let count = map.entry(word).or_insert(0);
  *count += 1;  // 반환된 &mut V로 값 수정
  ```
- `.get(key)` → `Option<&V>`로 접근
- `for (k, v) in &map`: 순회 (순서 무작위)
- `HashMap::with_capacity(n)`: 초기 용량 지정으로 재할당 최소화
- **해시 함수**: 기본적으로 **SipHash** 사용 — DoS 공격 저항성 우선 (성능보다 보안)

## 실행

```bash
rustc vectors.rs && ./vectors
rustc strings.rs && ./strings
rustc hashmaps.rs && ./hashmaps
```
