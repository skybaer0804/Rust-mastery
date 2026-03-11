# 07. Modules

## 핵심 개념

### 모듈 시스템
- `mod`: 모듈 정의
- `pub`: 공개 접근 제어자
- `use`: 경로를 스코프로 가져오기
- `as`: 별칭 지정
- `self`, `super`: 상대 경로

### 경로
- **절대 경로**: `crate::module::function`
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

## 실행

```bash
rustc modules_example.rs && ./modules_example
```
