// 제네릭 (Generics)

// 제네릭 함수
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 제네릭 구조체
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 서로 다른 타입의 제네릭 구조체
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// 제네릭 구조체의 메서드
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 특정 타입에만 메서드 구현
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 다른 타입 매개변수를 가진 메서드
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

// 제네릭 enum (표준 라이브러리 예시)
// enum Option<T> { Some(T), None }
// enum Result<T, E> { Ok(T), Err(E) }

fn main() {
    // 제네릭 함수
    let number_list = vec![34, 50, 25, 100, 65];
    println!("가장 큰 수: {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("가장 큰 문자: {}", largest(&char_list));

    // 제네릭 구조체
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("정수 포인트: {:?}", integer_point);
    println!("실수 포인트: {:?}", float_point);

    // 메서드 호출
    println!("integer_point.x = {}", integer_point.x());
    println!("원점까지 거리: {}", float_point.distance_from_origin());

    // 다른 타입의 제네릭
    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("혼합 포인트: {:?}", mixed);

    // mixup
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("mixup 결과: x={}, y={}", p3.x, p3.y);
}
