fn main() {
    // dangling references - a pointer/reference that still exists, but the data it points to is already gone.
    // example
    // u borrow a book from library
    // the library through the book away
    // but u still keep the borrow card

    // this code is error
    let l = dangle();

    // in line 17 s is the owner of the stack which consist pointer, length, and capacity
    // but in line 18 s droped no longer s in the memory
    // but l still borrow the pointer from the s which is null
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}