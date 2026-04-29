fn main() {
    let mut s = String::from("hello world");
    let l = first_word(&mut s);
    println!("{l}")
}

fn first_word(s: &String) -> &str {
    &s[0..5]
}