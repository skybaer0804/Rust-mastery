# 05. Structs

## 핵심 개념

### 구조체 정의 (structs.rs)
- `struct` 키워드로 정의 — 관련 데이터를 묶어 의미 있는 타입 생성
- 필드에 이름과 타입 지정
- **필드 초기화 축약**: 변수명과 필드명이 같으면 축약 가능
  ```rust
  fn build_user(email: String, username: String) -> User {
      User { email, username, active: true }  // email: email 대신 email
  }
  ```
- **구조체 업데이트 문법**: `..other_instance`로 나머지 필드 복사
  ```rust
  let user2 = User { email: String::from("new@example.com"), ..user1 };
  // 주의: ..user1로 소유권이 이동하면 user1은 사용 불가할 수 있음
  ```
- **튜플 구조체**: `struct Color(i32, i32, i32)` — 필드 이름 없음, 인덱스로 접근
- **유닛 구조체**: `struct AlwaysEqual;` — 필드 없음, 트레이트 구현 시 유용

### 디버그 출력
```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

println!("{:?}", rect);   // 한 줄 출력: Rectangle { width: 30, height: 50 }
println!("{:#?}", rect);  // 여러 줄 보기 좋게 출력
```
- `#[derive(Debug)]` 어트리뷰트로 `Debug` 트레이트 자동 구현
- `dbg!()` 매크로: 값과 파일/줄 번호 출력, 소유권을 돌려줌
  ```rust
  let rect = dbg!(Rectangle { width: 30, height: 50 });  // stderr에 출력
  ```

### 구조체와 소유권
- 구조체는 자신의 데이터를 **소유**하는 것이 일반적
- 구조체 필드에 참조(`&str`)를 넣으려면 **라이프타임** 어노테이션 필요
  ```rust
  // struct User { username: &str }  // 에러: 라이프타임 지정 필요
  struct User<'a> { username: &'a str }  // OK
  ```

### 메서드 (methods.rs)
- `impl` 블록 안에 정의
- 첫 번째 매개변수는 `&self`(불변 참조), `&mut self`(가변 참조), `self`(소유권 이동)
- **연관 함수**: `self` 없이 정의 — 생성자 패턴에 자주 사용
  ```rust
  impl Rectangle {
      fn new(width: u32, height: u32) -> Self {
          Self { width, height }
      }
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  let rect = Rectangle::new(30, 50);  // :: 문법으로 호출
  ```
- 여러 `impl` 블록 가능 (10장 제네릭에서 활용)

## 실행

```bash
rustc structs.rs && ./structs
rustc methods.rs && ./methods
```
