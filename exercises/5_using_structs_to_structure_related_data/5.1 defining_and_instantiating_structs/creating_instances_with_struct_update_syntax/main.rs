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
        email: String::from("rust@mail.com"),
        sign_in_count: 1
    };
    
    let new_user = User {
        email: String::from("rust@gmail.com"),
        ..user
    };
    
    println!("active -> {}", new_user.active);
    println!("username -> {}", new_user.username);
    println!("email -> {}", new_user.email);
    println!("sign in count -> {}", new_user.sign_in_count);
    
}