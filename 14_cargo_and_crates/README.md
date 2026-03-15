# 14. Cargo and Crates

## 핵심 개념

### 릴리스 프로파일
```toml
[profile.dev]
opt-level = 0  # 빠른 컴파일, 최적화 없음 (기본값)

[profile.release]
opt-level = 3  # 느린 컴파일, 최대 최적화 (기본값)
```
- `cargo build`: dev 프로파일 → 개발 중 반복 빌드에 최적
- `cargo build --release`: release 프로파일 → 배포용 성능 최적화

### 문서화
```rust
/// 두 수를 더합니다.
///
/// # Examples
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
/// 오버플로우 시 패닉
///
/// # Errors
/// 실패 조건 설명
fn add(a: i32, b: i32) -> i32 { a + b }
```
- `///`: 아래 항목 문서 주석 (Markdown 지원)
- `//!`: 파일/모듈 전체 문서 (`src/lib.rs` 상단에 크레이트 설명)
- `cargo doc --open`: 문서 생성 및 브라우저에서 열기
- **문서 테스트**: `# Examples` 코드 블록은 `cargo test` 시 자동 실행

### pub use로 공개 API 설계
```rust
// 내부 구조가 깊어도 편리한 경로로 re-export
pub use self::kinds::PrimaryColor;
pub use self::utils::mix;
```
- 내부 구현 경로(`a::b::c::Type`)와 다른 공개 API 경로 제공 가능

### crates.io 배포
- `cargo login <token>`: API 토큰 등록
- `cargo publish`: 크레이트 배포 (한 번 배포하면 버전 삭제 불가)
- `cargo yank --vers x.y.z`: 버전 비활성화 (기존 프로젝트는 유지, 신규 사용만 차단)

### 바이너리 설치
```bash
cargo install ripgrep    # crates.io에서 바이너리 크레이트 설치
cargo install --path .   # 로컬 크레이트 설치
# ~/.cargo/bin/ 에 설치됨
```

### 워크스페이스
여러 관련 패키지를 하나의 디렉토리에서 관리
```toml
# Cargo.toml (루트)
[workspace]
members = ["adder", "add_one"]
```
- 공유 `target/` 디렉토리, 공유 `Cargo.lock` → 의존성 버전 일치 보장
- 워크스페이스 내 크레이트 간 참조: `add_one = { path = "../add_one" }`

### 버전 범위 지정
| 표기 | 의미 |
|---|---|
| `"0.8.5"` | `>=0.8.5, <0.9.0` (SemVer 호환 업데이트 허용) |
| `"=0.8.5"` | 정확히 0.8.5만 |
| `">=0.8, <1.0"` | 명시적 범위 |
| `"*"` | 모든 버전 |

### 카고 확장
```bash
cargo fmt      # 코드 포맷팅
cargo clippy   # 린트 검사
cargo-xxx      # cargo-xxx 바이너리가 있으면 cargo xxx로 실행 가능
```

## 실행

```bash
rustc cargo_example.rs && ./cargo_example
```
