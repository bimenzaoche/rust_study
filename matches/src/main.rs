#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
fn main() {
    // println!("Hello, world!");
    // let state1 =  value_in_cents(Coin::Quarter(UsState::Alaska));
    // println!("{}", state1);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // println!("five {:?}", five);
    // println!("six {:?}", six);
    // println!("none {:?}", none);

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => (),
    //     // _ => reroll(),
    //     // other => move_player(other),
    // }

    // let config_max = Some(3u8);
    // // match config_max {
    // //     Some(max) => println!("The maximum is configured to be {}", max),
    // //     _ => (),
    // // }
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }

    let mut count = 0;
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}


