# 02. Guessing Game

## 핵심 개념

### 외부 크레이트 사용
- `rand` 크레이트로 난수 생성
- `Cargo.toml`의 `[dependencies]`에 추가
```toml
[dependencies]
rand = "0.8.5"
```
- **SemVer(유의적 버전)**: `"0.8.5"`는 `>=0.8.5, <0.9.0` 범위를 의미
- **`Cargo.lock`**: 처음 빌드 시 확정된 버전을 고정 기록 → 재현 가능한 빌드 보장
  - `cargo update`로 lock 파일 갱신 가능

### 트레이트 임포트
```rust
use rand::Rng;  // Rng 트레이트를 스코프에 포함해야 메서드 사용 가능
```
- 트레이트를 `use`로 가져와야 해당 트레이트의 메서드가 스코프에 들어옴

### 주요 문법
- `let mut`: 가변 변수 선언
- `String::new()`: 빈 문자열 생성 (연관 함수)
- `.read_line(&mut guess)`: 입력 받기 (메서드) — `&mut`으로 가변 참조 전달
- `match`: 패턴 매칭으로 분기 처리
- `loop`: 무한 루프
- `.parse()`: 문자열을 숫자로 변환
- `Ordering`: 비교 결과 (`Less`, `Greater`, `Equal`)
- `{}`: 출력 자리표시자(placeholder)

### 섀도잉(Shadowing)으로 타입 변환
```rust
let guess = String::new();
// ...입력 받기...
let guess: u32 = guess.trim().parse().expect("숫자를 입력하세요!");
```
- 같은 이름 `guess`를 재선언하여 `String` → `u32` 타입 변환
- 새로운 변수를 만들지 않아도 됨 (타입 변경도 가능)

### match로 에러 복구
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,  // 숫자가 아니면 루프 재시작
};
```
- `.expect()` 대신 `match`를 사용하면 에러 시 프로그램을 종료하지 않고 복구 가능
- `Err(_)`: 에러 값을 무시하고 모든 에러를 처리

### 에러 처리 맛보기
- `Result` 타입: `Ok`, `Err`
- `.expect()`: 에러 시 메시지 출력 후 종료 (`unwrap` + 커스텀 메시지)

## 실행

> 이 예제는 `rand` 크레이트 없이 간소화한 버전입니다.

```bash
rustc guessing_game.rs && ./guessing_game
```

> Cargo 프로젝트로 실행 시:
```bash
cargo run
```
