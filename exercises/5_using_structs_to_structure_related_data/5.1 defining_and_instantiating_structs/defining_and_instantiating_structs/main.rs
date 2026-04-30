struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user = User {
        active: true,
        username: String::from("rust"),
        email: String::from("rust@gmail.com"),
        sign_in_count: 1
    };
    
    println!("active -> {0}", user.active);
    println!("username -> {0}", user.username);
    println!("email -> {0}", user.email);
    println!("sign in count -> {0}", user.sign_in_count);
}