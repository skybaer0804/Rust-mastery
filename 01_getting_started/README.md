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

### AOT(Ahead-of-Time) 컴파일
- Rust는 AOT 컴파일 언어로, **컴파일과 실행이 별개**
- 컴파일 결과물인 실행 파일은 **Rust가 설치되지 않은 환경에서도 실행 가능**
- 반면 인터프리터 언어(Python, Ruby 등)는 실행 시점에 런타임이 필요

### Cargo
Cargo는 Rust의 빌드 시스템 및 패키지 매니저로, 대부분의 Rust 프로젝트에서 사용된다.
- **의존성(dependency)**: 프로젝트에서 사용하는 외부 코드 패키지 → Rust에서는 **크레이트(crate)** 라고 부름

#### 주요 명령어
| 명령어 | 설명 |
|---|---|
| `cargo --version` | 설치 여부 확인 |
| `cargo new <이름>` | 새 프로젝트 생성 (src/main.rs + Cargo.toml 자동 생성) |
| `cargo build` | 디버그 빌드 (`target/debug/`에 실행 파일 생성) |
| `cargo build --release` | 최적화된 릴리즈 빌드 (`target/release/`에 생성) |
| `cargo run` | 빌드 + 실행 (변경 없으면 재빌드 생략) |
| `cargo check` | 컴파일 가능 여부만 확인 (빠름, 실행 파일 미생성) |

#### 프로젝트 구조
```
hello_cargo/
├── Cargo.toml     # 패키지 메타데이터 및 의존성 설정
├── Cargo.lock     # 의존성 버전 고정 기록 (자동 관리)
├── src/
│   └── main.rs   # 소스 코드
└── target/        # 빌드 결과물
```

#### Cargo.toml
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```
- `[package]`: 패키지 이름, 버전, 에디션
- `[dependencies]`: 사용할 크레이트 목록

#### 빌드 모드 차이
- **debug 빌드** (`cargo build`): 빠른 컴파일, 최적화 없음 → 개발 중 사용
- **release 빌드** (`cargo build --release`): 느린 컴파일, 실행 성능 최적화 → 배포 시 사용

#### 기존 프로젝트 빌드
```bash
git clone <url>
cd <project>
cargo build
```
모든 OS에서 동일한 명령어로 빌드 가능 (Cargo가 의존성 자동 처리)

## 실행

```bash
# rustc 직접 사용
rustc hello_world.rs && ./hello_world

# Cargo 사용 (권장)
cargo run
```
