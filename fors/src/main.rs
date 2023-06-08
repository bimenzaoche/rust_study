fn main() {
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    
    // while index < 5 {
    //     println!("{}", a[index]);
    //     index += 1;
    // }

    // for element in a {
    //     println!("{element}");
    // }

    // for number in (1..4).rev() {
    //     println!("{number}");
    // }
    // println!("end");

    // let temp = 20.0;
    // let hs_temp = ss_to_hs(temp);
    // println!("华氏温度{hs_temp}");
    // let hs_temp = 30.0;
    // let temp = hs_to_ss(hs_temp);
    // println!("摄氏温度{temp}");

    // println!("费波拉契0的值是{}", fb(0));
    // println!("费波拉契1的值是{}", fb(1));
    // println!("费波拉契2的值是{}", fb(2));
    // println!("费波拉契3的值是{}", fb(3));
    // println!("费波拉契4的值是{}", fb(4));
    // println!("费波拉契99的值是{}", fb(50));

    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() 在字符串后追加字面值

    // println!("{}", s); // 将打印 `hello, world!`
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);

}

// fn hs_to_ss(num:f64) -> f64 {
//     (num-20.0)/1.8
// }

// fn ss_to_hs(num:f64) -> f64 {
//     num*1.8+20.0
// }

// fn fb(num:u32) -> u64 {
//     if num>1 {
//         fb(num-1) + fb(num-2)
//     } else {
//         1
//     }
    
// }