# 13. Closures and Iterators

## 핵심 개념

### 클로저 (closures.rs)
- `|x| x + 1`: 익명 함수, 환경 캡처 가능
- 타입 추론 (첫 사용 시 타입 고정)
- **캡처 방식**:
  - `Fn`: 불변 참조로 캡처
  - `FnMut`: 가변 참조로 캡처
  - `FnOnce`: 소유권 가져감 (한 번만 호출 가능)
- `move` 키워드: 강제로 소유권 가져감

### 이터레이터 (iterators.rs)
- `Iterator` 트레이트: `fn next(&mut self) -> Option<Self::Item>`
- **소비 어댑터**: `.collect()`, `.sum()`, `.count()`
- **이터레이터 어댑터**: `.map()`, `.filter()`, `.zip()`
- 지연 평가(lazy): `.collect()` 등 호출 전까지 실행 안 됨
- **제로 비용 추상화**: for 루프와 동일 성능

## 실행

```bash
rustc closures.rs && ./closures
rustc iterators.rs && ./iterators
```
