
// enum IpAddKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddKind,
//     address: String,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    

    // let home = IpAddr {
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    
}
