# 12. Minigrep Project

## 핵심 개념

### CLI 프로젝트 구조
- `std::env::args()`: 커맨드라인 인자 읽기
- `std::fs::read_to_string()`: 파일 읽기
- 관심사 분리: `main.rs` vs `lib.rs`

### 리팩토링 원칙
- 함수 추출로 각 기능 분리
- 에러 메시지 개선 (`expect` → `Result`)
- `process::exit()`로 적절한 종료 코드

### 환경 변수
- `std::env::var()`: 환경 변수 읽기
- 대소문자 구분 검색 / 무시 옵션

### 표준 에러
- `eprintln!`: stderr로 출력
- `println!`: stdout로 출력

## 실행

```bash
rustc minigrep.rs && ./minigrep
```
