fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}")

    // as i descibed before s is stack (like stack of plate)

    // world  ^
    // hello  |
}