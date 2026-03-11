// 모듈 시스템

// 모듈 정의
mod front_of_house {
    // pub으로 공개
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("대기 목록에 추가");
        }

        fn seat_at_table() {
            println!("테이블 배정");
        }
    }

    mod serving {
        fn take_order() {
            println!("주문 받기");
        }

        fn serve_order() {
            println!("음식 서빙");
        }
    }
}

mod back_of_house {
    // pub 구조체: 구조체는 공개, 필드는 개별 pub 필요
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // 비공개
    }

    impl Breakfast {
        // 비공개 필드가 있으므로 생성자 함수 필요
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    // pub enum: 모든 배리언트가 공개됨
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn fix_incorrect_order() {
        cook_order();
        // super로 부모 모듈 접근
        super::deliver_order();
    }

    fn cook_order() {
        println!("요리 중...");
    }
}

fn deliver_order() {
    println!("배달 중...");
}

// use로 경로 가져오기
use front_of_house::hosting;

fn main() {
    // 절대 경로
    front_of_house::hosting::add_to_waitlist();

    // use를 통한 호출
    hosting::add_to_waitlist();

    // 구조체 사용
    let mut meal = back_of_house::Breakfast::summer("호밀");
    meal.toast = String::from("밀"); // pub 필드 변경 가능
    println!("토스트: {}", meal.toast);
    // meal.seasonal_fruit = String::from("블루베리"); // 에러! 비공개 필드

    // enum 사용
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("전채: {:?}, {:?}", order1, order2);

    // super 사용 예
    back_of_house::fix_incorrect_order();
}
