// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

// 定义
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("新顾客来了！");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
    
        fn server_order() {}
    
        fn take_payment() {}
    }
}

// 路径引用
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

// 结构体，枚举等默认都是私有
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        super::front_of_house::hosting::add_to_waitlist();
    }

    fn cook_order() {}

    pub struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        } 
    }
}

pub fn eat() {
    let mut meal = back_of_house::Breakfast::summer(String::from("Rye"));
    
}

// use导入
use front_of_house::hosting;
// pub use front_of_house::hosting;  // 重导入
use std::collections::HashMap;
use std::fmt::Result as FmnResult;
use std::{cmp::Ordering, io::Result as IoResult};
use std::io::{self, Write};
use std::collections::*;

// 分割模块到不同的文件
mod home;
pub use crate::home::bed;

pub fn sleep() {
    bed::sleep();
}

// pub mod house;
// pub use crate::house::bed;




