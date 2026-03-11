// 메서드 (Methods)

#[derive(Debug)] // {:?}로 출력 가능하게 함
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 블록 — 메서드 정의
impl Rectangle {
    // 메서드: &self를 첫 번째 매개변수로
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // 다른 Rectangle을 매개변수로 받는 메서드
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 연관 함수 (Associated Function) — self 없음
    // String::from과 같은 패턴
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Debug 출력
    println!("rect1: {:?}", rect1);
    println!("rect1 (pretty): {:#?}", rect1);

    // 메서드 호출
    println!("넓이: {}", rect1.area());
    println!("둘레: {}", rect1.perimeter());

    // 다른 사각형과 비교
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1이 rect2를 포함? {}", rect1.can_hold(&rect2)); // true
    println!("rect1이 rect3를 포함? {}", rect1.can_hold(&rect3)); // false

    // 연관 함수 호출 (:: 문법)
    let square = Rectangle::square(20);
    println!("정사각형: {:?}, 넓이: {}", square, square.area());
}
