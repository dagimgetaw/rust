fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!"); // this code is error -- compiling ownership v0.1.0

    // fix 
    let s2 = s1.clone(); 
    println!("{s1}, world!");

}