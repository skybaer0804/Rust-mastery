# 10. Generics, Traits, Lifetimes

## 핵심 개념

### 제네릭 (generics.rs)
- 함수, 구조체, enum, 메서드에 타입 매개변수 사용
- `fn largest<T: PartialOrd>(list: &[T]) -> &T`
- 컴파일 타임에 **단형성화(monomorphization)** → 런타임 비용 없음

### 트레이트 (traits.rs)
- 공유 동작을 정의하는 인터페이스
- `trait Summary { fn summarize(&self) -> String; }`
- `impl Trait for Type`으로 구현
- **기본 구현**: 트레이트에 기본 메서드 제공
- **트레이트 바운드**: `fn notify(item: &impl Summary)` 또는 `fn notify<T: Summary>(item: &T)`
- `+` 로 여러 트레이트 결합: `T: Summary + Display`
- `where` 절로 가독성 향상
- **반환 타입**: `-> impl Summary`

### 라이프타임 (lifetimes.rs)
- 참조의 유효 범위를 명시
- `'a`: 라이프타임 어노테이션
- `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
- **구조체의 라이프타임**: 참조를 필드로 가질 때 필요
- **라이프타임 생략 규칙** (3가지)
- `'static`: 프로그램 전체 기간

## 실행

```bash
rustc generics.rs && ./generics
rustc traits.rs && ./traits
rustc lifetimes.rs && ./lifetimes
```
