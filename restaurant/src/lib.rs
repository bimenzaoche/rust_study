// mod front_of_house {
    

//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// // pub fn eat_at_restaurant() {
// //     //绝对路径
// //     crate::front_of_house::hosting::add_to_waitlist();
// //     //相对路径
// //     front_of_house::hosting::add_to_waitlist();
// // }

// fn deliver_order() {}

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}

//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub use crate::front_of_house::hosting;
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     // 如果取消下一行的注释代码不能编译；
//     // 不允许查看或修改早餐附带的季节水果
//     // meal.seasonal_fruit = String::from("blueberries");

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }


mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}