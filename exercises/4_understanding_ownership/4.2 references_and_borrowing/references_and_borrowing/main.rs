fn main() {
    let s1 = String::from("hello"); // s1 owns the data
    let s2 = s1; // ownership moves to s2

    // println!("{s1}"); error: s1 is no longer valid

    println!("{s2}"); // ok

    let r1 = String::from("hello"); // s1 owns data
    let r2 = &r1; // r1 borrows s1 (no ownership change)

    println!("{r1}"); // still valid
    println!("{r2}"); // also valid
}