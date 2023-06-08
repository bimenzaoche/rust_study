fn main() {
    // let number = 6;

    // // if number <5 {
    // //     println!("条件正确！");
    // // } else {
    // //     println!("条件错误！");
    // // }
    // if number !=0 {
    //     println!("不为0");
    // } 

    // let number = 12;
    // if number % 4 == 0 {
    //     println!("可以被4整除");
    // } else if number % 3 == 0 {
    //     println!("可以被3整除");
    // } else if number % 2 == 0 {
    //     println!("可以被2整除");
    // } else {
    //     println!("不能被2、3、4整除");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
