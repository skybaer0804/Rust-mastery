# 15. Smart Pointers

## 핵심 개념

### Box<T> (box_example.rs)
- 힙에 데이터 할당
- 사용 시점: 컴파일 타임에 크기를 모를 때, 큰 데이터 이동 없이 소유권 전달, 트레이트 객체
- 재귀적 타입 정의에 필수 (예: cons list)

### Rc<T> (rc_example.rs)
- **참조 카운팅**: 여러 소유자 허용 (단일 스레드)
- `Rc::clone(&rc)`: 참조 카운트 증가 (깊은 복사 아님)
- `Rc::strong_count()`: 현재 참조 수 확인
- 불변 참조만 제공

### RefCell<T> (refcell_example.rs)
- **내부 가변성**: 불변 참조를 통해 값 변경 가능
- 빌림 규칙을 **런타임**에 검사
- `.borrow()`: 불변 참조
- `.borrow_mut()`: 가변 참조
- 규칙 위반 시 런타임 panic

### Deref와 Drop
- `Deref` 트레이트: `*` 연산자 커스터마이징, 역참조 강제 변환
- `Drop` 트레이트: 스코프 벗어날 때 정리 코드 실행
- `std::mem::drop()`: 조기 해제

## 실행

```bash
rustc box_example.rs && ./box_example
rustc rc_example.rs && ./rc_example
rustc refcell_example.rs && ./refcell_example
```
