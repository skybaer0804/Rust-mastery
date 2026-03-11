// 데이터 타입

fn main() {
    // === 정수형 ===
    let a: i32 = -42;       // 부호 있는 32비트
    let b: u8 = 255;        // 부호 없는 8비트
    let c = 1_000;          // 숫자 구분자 (_)
    let d = 0xff;           // 16진수
    let e = 0o77;           // 8진수
    let f = 0b1111_0000;    // 2진수
    let g = b'A';           // 바이트 (u8)
    println!("정수: {} {} {} {} {} {} {}", a, b, c, d, e, f, g);

    // === 부동소수점 ===
    let x = 2.0;            // f64 (기본)
    let y: f32 = 3.0;       // f32
    println!("부동소수점: {} {}", x, y);

    // === 수학 연산 ===
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("연산: {} {} {} {} {}", sum, difference, product, quotient, remainder);

    // === 불리언 ===
    let t = true;
    let f: bool = false;
    println!("불리언: {} {}", t, f);

    // === 문자 (4바이트, 유니코드) ===
    let c = 'z';
    let heart = '❤';
    let hangul = '가';
    println!("문자: {} {} {}", c, heart, hangul);

    // === 튜플 — 고정 길이, 여러 타입 ===
    let tup: (i32, f64, char) = (500, 6.4, '가');
    let (tx, ty, tz) = tup;  // 구조분해
    println!("튜플: {} {} {}", tx, ty, tz);
    println!("인덱스 접근: {}", tup.0);

    // === 배열 — 고정 길이, 같은 타입, 스택 할당 ===
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("배열: {:?} {:?} {:?}", arr, arr2, arr3);
    println!("배열 접근: {}", arr[0]);
}
