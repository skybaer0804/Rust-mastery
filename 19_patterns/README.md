# 19. Patterns

## 핵심 개념

### 패턴이 사용되는 곳
- `match` arms
- `if let`, `while let`
- `for` 루프
- `let` 구문
- 함수 매개변수

### 반박 가능성
- **반박 불가능(irrefutable)**: 항상 매칭 (예: `let x = 5`)
- **반박 가능(refutable)**: 매칭 실패 가능 (예: `if let Some(x) = value`)

### 패턴 문법
- 리터럴, 변수, 와일드카드 (`_`)
- `|`: 여러 패턴 (or)
- `..=`: 범위 패턴 (`1..=5`)
- 구조분해: 구조체, enum, 튜플, 참조
- `..`: 나머지 무시
- **매치 가드**: `if` 조건 추가
- `@` 바인딩: 패턴 매칭 + 변수 바인딩 동시에

## 실행

```bash
rustc patterns.rs && ./patterns
```
