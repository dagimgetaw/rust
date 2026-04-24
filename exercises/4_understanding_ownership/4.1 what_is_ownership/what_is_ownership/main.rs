fn main() {
    // ownership is a set of rules that govern how a Rust program manages memory.
    // some languages have garbage collection that regularly looks for no-longer-used memory as the program runs.
    // rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks.

    // a garbage collector is a system that - automatically finds and deletes memory you no longer use.
    // simple idea
    // imagine you - borrow books from a library and forget to return them
    // a garbage collector is like a librarian who - checks what books you’re no longer using and takes them back automatically.

    // ownership rule
    // one - each value in rust has an own owner.
    // two - there can only be one owner at a time.
    // three - when the owner goes out of scope, the value will be droped.
}