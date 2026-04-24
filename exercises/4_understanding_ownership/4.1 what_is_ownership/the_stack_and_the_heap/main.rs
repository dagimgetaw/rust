fn main() {
    // stack
    // think of the stack like a stack of plates you can only add/remove from the top.
    // it is very fast.
    // store fixed data size.

    // heap
    // think of the heap like a big messy warehouse.
    // data can be anywhere.
    // needs a pointer to find it.
    // used for dynamic size data.

    // example 
    let s = String::from("hello");

    // s is stack it have three part
    // one - pointer to heap
    // two - length
    // three - capacity 
    // heap = "hello"

    // simple explanation
    // stack - actual value
    // heap - data stored elsewhere
    // pointer - link to heap
}