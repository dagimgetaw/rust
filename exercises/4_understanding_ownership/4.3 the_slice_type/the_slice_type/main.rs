fn main() {
    // slices in rust are basically a “view” into a collection (like an array or String), not the actual owner of the data.
    // example - you’re looking at a part of something, but you don’t own it.

    let s = String::from("hello world");

    let hello = &s[..5];   // from start to index 5
    let world = &s[6..];   // from index 6 to end
    let full  = &s[..];    // whole string

    println!("{}", hello); // hello
    println!("{}", world); // world
    println!("{}", full); // hello world

}