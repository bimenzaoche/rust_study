fn main() {
    // let v: Vec<i32> = Vec::new();
    
    // let v = vec![1, 2, 3];

    // let mut v: Vec<i32> = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("第三个元素是{third}");

    // let thrid: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("第三个元素是{third}"),
    //     None => println!("没有第三个元素"),
    // }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("第一个是{first}");

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

}
