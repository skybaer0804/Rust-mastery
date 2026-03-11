# 02. Guessing Game

## 핵심 개념

### 외부 크레이트 사용
- `rand` 크레이트로 난수 생성
- `Cargo.toml`의 `[dependencies]`에 추가

### 주요 문법
- `let mut`: 가변 변수 선언
- `String::new()`: 빈 문자열 생성
- `io::stdin().read_line()`: 사용자 입력 받기
- `match`: 패턴 매칭으로 분기 처리
- `loop`: 무한 루프
- `.parse()`: 문자열을 숫자로 변환
- `Ordering`: 비교 결과 (Less, Greater, Equal)

### 에러 처리 맛보기
- `Result` 타입: `Ok`, `Err`
- `.expect()`: 에러 시 메시지 출력 후 종료

## 실행

> 이 예제는 `rand` 크레이트 없이 간소화한 버전입니다.

```bash
rustc guessing_game.rs && ./guessing_game
```
