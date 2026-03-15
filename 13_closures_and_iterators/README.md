# 13. Closures and Iterators

함수형 언어의 영향을 받은 기능들 — **제로 비용 추상화**로 `for` 루프와 동일한 성능 보장

## 핵심 개념

### 클로저 (closures.rs)
변수에 저장하거나 인자로 전달할 수 있는 익명 함수
```rust
let add_one = |x: i32| -> i32 { x + 1 };  // 전체 문법
let add_one = |x| x + 1;                   // 타입 추론 버전
```
- **타입 고정**: 첫 호출 시 타입이 확정 → 이후 다른 타입으로 호출 불가
- 함수와 달리 **주변 환경(scope)을 캡처** 가능

#### 캡처 방식 (컴파일러가 자동으로 최소 비용 방식 선택)
| 트레이트 | 캡처 방식 | 특징 |
|---|---|---|
| `Fn` | 불변 참조 | 여러 번 호출 가능 |
| `FnMut` | 가변 참조 | 여러 번 호출 가능 |
| `FnOnce` | 소유권 이동 | 한 번만 호출 가능 |

- `move` 키워드: 강제로 소유권 가져감 — **스레드에 클로저 전달 시 필수**
  ```rust
  thread::spawn(move || println!("{list:?}"));  // list 소유권을 스레드로 이동
  ```

### 이터레이터 (iterators.rs)
- `Iterator` 트레이트: `fn next(&mut self) -> Option<Self::Item>` 구현 필수
- `next()`만 구현하면 나머지 메서드(`map`, `filter`, `zip`, `enumerate` 등) 모두 제공
- **지연 평가(lazy)**: 소비 어댑터 호출 전까지 실행 안 됨

#### 소비 어댑터 (이터레이터 소비, `next()` 호출)
```rust
v.iter().sum::<i32>()       // 합계
v.iter().count()             // 개수
v.iter().collect::<Vec<_>>() // 컬렉션으로 수집
```

#### 이터레이터 어댑터 (새 이터레이터 반환, lazy)
```rust
v.iter().map(|x| x + 1)
        .filter(|x| x % 2 == 0)
        .zip(other.iter())
        .enumerate()              // (index, value) 튜플
        .take(5)                  // 처음 5개만
        .skip(2)                  // 처음 2개 건너뜀
        .flat_map(|x| vec![x, x]) // 중첩 해제
        .fold(0, |acc, x| acc + x) // 누적 연산
        .collect::<Vec<_>>()
```

#### 커스텀 이터레이터
```rust
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 { self.count += 1; Some(self.count) }
        else { None }
    }
}
```

## 실행

```bash
rustc closures.rs && ./closures
rustc iterators.rs && ./iterators
```
