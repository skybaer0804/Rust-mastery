# 12. Minigrep Project

7~11장에서 배운 내용(코드 조직화, 컬렉션, 에러 처리, 트레이트, 라이프타임, 테스트)을 종합 적용하는 CLI 프로젝트.

## 핵심 개념

### CLI 프로젝트 구조
- `std::env::args()`: 커맨드라인 인자 읽기 → 이터레이터 반환
- `std::fs::read_to_string()`: 파일 전체를 `String`으로 읽기
- **관심사 분리**: `main.rs`(진입점/설정) vs `lib.rs`(비즈니스 로직)
  - `main.rs`: 인자 파싱, 에러 처리, `process::exit()` 호출
  - `lib.rs`: `run()`, `search()` 등 핵심 로직 — 테스트 용이

### Config 구조체로 인자 파싱
```rust
struct Config { query: String, file_path: String }

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("인자가 부족합니다"); }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
    }
}
```

### 에러 처리 패턴
```rust
// main.rs
fn main() {
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("인자 오류: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("애플리케이션 에러: {e}");
        process::exit(1);
    }
}
```
- `Box<dyn Error>`: 여러 에러 타입을 하나로 처리

### search 함수와 라이프타임
```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 반환하는 &str 슬라이스는 contents에서 온 것 → contents의 라이프타임 연결
    contents.lines().filter(|line| line.contains(query)).collect()
}
```
- 라이프타임 어노테이션으로 반환값이 `query`가 아닌 `contents`에서 왔음을 명시

### TDD(테스트 주도 개발) 흐름
1. 실패하는 테스트 작성
2. `search` 함수 로직 구현
3. 테스트 통과 확인 → 리팩토링

### 환경 변수
```bash
IGNORE_CASE=1 cargo run -- query file.txt
```
```rust
let ignore_case = std::env::var("IGNORE_CASE").is_ok();
```

### 표준 에러
- `eprintln!`: **stderr** 출력 — 파이프/리디렉션 시에도 터미널에 표시
- `println!`: **stdout** 출력 — `>` 리디렉션으로 파일에 저장 가능
  ```bash
  cargo run -- query file.txt > output.txt  # 에러만 화면에 표시
  ```

## 실행

```bash
rustc minigrep.rs && ./minigrep
# Cargo 프로젝트로:
cargo run -- searchstring poem.txt
```
