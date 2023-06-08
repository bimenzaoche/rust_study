fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("cooper"),
    //     email: String::from("sun#163.com"),
    //     sign_in_count: 1,
    // };
    let mut user1 = build_user(String::from("sun#163.com"), String::from("cooper"));
    // user1.username = String::from("sun");
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    println!("{}", user2.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        // username: username,
        email,
        // email: email,
        sign_in_count: 1,
    }
    
}
