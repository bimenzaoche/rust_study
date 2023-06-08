// use std::io;
// use std::cmp::Ordering;
use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("猜数字!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is:{}", secret_number);
    loop {
        println!("请输入你猜想的数字。");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("只允许输入1到100之间的整数");
            continue;
        }
        println!("你猜的数字是:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小!"),
            Ordering::Greater => println!("太大!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
    
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("只允许输入1到100之间的整数 {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
