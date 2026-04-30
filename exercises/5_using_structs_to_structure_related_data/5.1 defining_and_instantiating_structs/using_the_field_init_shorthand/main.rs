struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user = build_user(
        true,
        String::from("rust"),
        String::from("rust@mail.com"),
        1
    );
    
    println!("active -> {}", user.active);
    println!("username -> {}", user.username);
    println!("email -> {}", user.email);
    println!("sign in count -> {}", user.sign_in_count);
    
}

fn build_user(active: bool, username: String, email: String, sign_in_count: u64) -> User {
    User {
        active,
        username,
        email,
        sign_in_count
    }
}