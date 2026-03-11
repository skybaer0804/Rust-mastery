# 14. Cargo and Crates

## 핵심 개념

### 릴리스 프로파일
- `cargo build`: dev 프로파일 (최적화 0)
- `cargo build --release`: release 프로파일 (최적화 3)
- `Cargo.toml`에서 `[profile.dev]`, `[profile.release]` 커스터마이징

### 문서화
- `///`: 문서 주석 (Markdown 지원)
- `//!`: 모듈/크레이트 전체 문서
- `cargo doc --open`: 문서 생성 및 열기
- 문서 테스트: 코드 블록이 자동으로 테스트됨

### crates.io 배포
- `cargo login`: API 토큰 등록
- `cargo publish`: 크레이트 배포
- `cargo yank --vers x.y.z`: 버전 비활성화

### 워크스페이스
- 여러 관련 패키지를 하나의 `Cargo.toml`로 관리
- `[workspace]` 섹션에 `members` 지정

## 실행

```bash
rustc cargo_example.rs && ./cargo_example
```
