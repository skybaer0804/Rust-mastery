# 07. Modules

## 핵심 개념

### 4가지 핵심 구성요소
| 개념 | 설명 |
|---|---|
| **패키지(Package)** | Cargo 기능 단위 — `Cargo.toml` + 하나 이상의 크레이트 |
| **크레이트(Crate)** | 컴파일 단위 — 바이너리 또는 라이브러리 |
| **모듈(Module)** | 코드 조직화 및 접근 제어 단위 |
| **경로(Path)** | 모듈 트리에서 항목을 찾는 방법 |

### 패키지 vs 크레이트
- **바이너리 크레이트**: `src/main.rs`가 크레이트 루트 → 실행 파일 생성
- **라이브러리 크레이트**: `src/lib.rs`가 크레이트 루트 → 다른 프로젝트에서 사용
- 한 패키지에 바이너리 크레이트는 여러 개 가능, 라이브러리는 최대 1개

### 모듈을 파일로 분리
```rust
// src/main.rs 또는 src/lib.rs
mod garden;  // → src/garden.rs 또는 src/garden/mod.rs 탐색
```
```
src/
├── main.rs
├── garden.rs          // mod garden; 선언 시 자동 탐색
└── garden/
    ├── mod.rs         // 또는 이 파일
    └── vegetables.rs  // mod vegetables; 선언 시 탐색
```

### 모듈 시스템
- `mod`: 모듈 정의
- `pub`: 공개 접근 제어자
- `use`: 경로를 스코프로 가져오기
- `as`: 별칭 지정
- `self`, `super`: 상대 경로

### 경로
- **절대 경로**: `crate::module::function` (`crate`로 시작)
- **상대 경로**: `self::module::function`, `super::function`

### 접근 제어
- 기본적으로 모든 항목은 **비공개(private)**
- 부모 모듈은 자식의 비공개 항목에 접근 불가
- 자식 모듈은 부모의 항목에 접근 가능
- `pub` 구조체: 구조체는 공개, 필드는 여전히 비공개 (개별 `pub` 필요)
- `pub` enum: enum 공개 시 모든 배리언트도 공개

### use 관용구
- 함수: 부모 모듈까지 `use` (예: `use crate::module`)
- 구조체/enum: 전체 경로 `use` (예: `use std::collections::HashMap`)
- **`pub use`**: re-export — 외부에서 이 경로로도 접근 가능
  ```rust
  pub use crate::kinds::PrimaryColor;  // 내부 경로를 외부에 다시 공개
  ```
- **glob 가져오기**: `use std::collections::*` (테스트나 prelude 모듈에서 사용)
- **중첩 경로**: `use std::{io, cmp::Ordering};`

## 실행

```bash
rustc modules_example.rs && ./modules_example
```
