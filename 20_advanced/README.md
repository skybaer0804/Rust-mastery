# 20. Advanced

## 핵심 개념

### Unsafe Rust (unsafe_example.rs)
- `unsafe` 블록에서 가능한 것:
  - 원시 포인터 역참조
  - unsafe 함수 호출
  - 가변 정적 변수 접근/수정
  - unsafe 트레이트 구현
- 빌림 검사기 비활성화가 아님 — 특정 작업만 허용

### 고급 트레이트
- **연관 타입**: `type Item`으로 구현 시 타입 지정
- **기본 제네릭 타입**: `<T = Self>`
- **완전 정규화 문법**: `<Type as Trait>::function()`
- **슈퍼트레이트**: `trait A: B`
- **뉴타입 패턴**: 외부 타입에 외부 트레이트 구현

### 매크로 (macros.rs)
- **선언적 매크로**: `macro_rules!` — 패턴 매칭 기반 코드 생성
- **절차적 매크로**: derive, attribute, function-like
- `#[derive(Debug)]` 등의 원리

## 실행

```bash
rustc unsafe_example.rs && ./unsafe_example
rustc macros.rs && ./macros
```
