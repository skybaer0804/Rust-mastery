# 09. Error Handling

## 핵심 개념

### panic! (panic_example.rs)
- 복구 불가능한 에러 발생 시 사용
- 스택 되감기(unwinding) 또는 즉시 종료(abort)
- `RUST_BACKTRACE=1`로 백트레이스 확인
- 배열 범위 초과 접근 시 자동 panic

### Result<T, E> (result_example.rs)
- `Ok(T)`: 성공
- `Err(E)`: 실패
- `match`로 처리
- `.unwrap()`: Ok면 값, Err면 panic
- `.expect("msg")`: unwrap + 커스텀 메시지

### ? 연산자
- `Result`를 반환하는 함수 안에서 사용
- `Err`면 자동으로 현재 함수에서 반환
- 에러 타입 자동 변환 (`From` 트레이트)
- 체이닝 가능: `File::open(path)?.read_to_string(&mut s)?`

### 에러 처리 가이드라인
- 프로토타입: `unwrap()`, `expect()` 사용 OK
- 라이브러리: `Result` 반환으로 호출자에게 위임
- 무효한 상태 방지: 커스텀 타입으로 유효성 검사

## 실행

```bash
rustc panic_example.rs && ./panic_example
rustc result_example.rs && ./result_example
```
