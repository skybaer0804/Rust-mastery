// 패턴 매칭 (Pattern Matching)

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
}

fn value_in_cents(coin: &Coin) -> u8 {
    // match는 모든 경우를 처리해야 함 (exhaustive)
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    // match 사용
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{}센트", value_in_cents(&coin));

    let penny = Coin::Penny;
    println!("{}센트", value_in_cents(&penny));

    // Option과 match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);

    // 와일드카드 (_) — 나머지 모든 경우
    let dice_roll = 9;
    match dice_roll {
        3 => println!("모자를 얻었습니다!"),
        7 => println!("모자를 잃었습니다!"),
        _ => println!("다시 굴리세요"), // 나머지 모든 값
    }

    // === if let — 하나의 패턴만 관심 있을 때 ===
    let config_max = Some(3u8);

    // match 버전
    match config_max {
        Some(max) => println!("최대값: {}", max),
        _ => (),
    }

    // if let 버전 (더 간결)
    if let Some(max) = config_max {
        println!("최대값 (if let): {}", max);
    }

    // if let + else
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = &coin {
        println!("Quarter from {:?}", state);
    } else {
        println!("Quarter가 아닙니다: {:?}", coin);
    }

    // === while let ===
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("while let: {}", top);
    }
}
