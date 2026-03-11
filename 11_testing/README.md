# 11. Testing

## 핵심 개념

### 테스트 작성
- `#[test]` 어트리뷰트로 테스트 함수 표시
- `assert!()`: 조건이 true인지 확인
- `assert_eq!()`, `assert_ne!()`: 값 비교
- `#[should_panic]`: panic이 발생해야 통과
- `Result<(), String>` 반환으로 `?` 사용 가능

### 테스트 실행
- `cargo test`: 모든 테스트 실행
- `cargo test 함수명`: 특정 테스트만 실행
- `#[ignore]`: 기본 실행에서 제외
- `cargo test -- --ignored`: 무시된 테스트만 실행

### 테스트 구조
- **단위 테스트**: `#[cfg(test)]` 모듈, 비공개 함수 테스트 가능
- **통합 테스트**: `tests/` 디렉토리에 별도 파일

## 실행

```bash
rustc --test testing_example.rs && ./testing_example
```
