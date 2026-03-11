# 01. Getting Started

## 핵심 개념

### Rust 설치
- `rustup`: Rust 버전 관리 도구
- `rustc`: Rust 컴파일러
- `cargo`: Rust 빌드 시스템 및 패키지 매니저

### Hello, World!
- `fn main()`: 프로그램 진입점
- `println!`: 매크로 (함수가 아님, `!`에 주의)
- 세미콜론(`;`)으로 문장 종료

### Cargo
- `cargo new`: 새 프로젝트 생성
- `cargo build`: 프로젝트 빌드
- `cargo run`: 빌드 + 실행
- `cargo check`: 컴파일 가능 여부만 확인 (빠름)
- `Cargo.toml`: 프로젝트 설정 파일 (TOML 형식)

## 실행

```bash
rustc hello_world.rs && ./hello_world
```
