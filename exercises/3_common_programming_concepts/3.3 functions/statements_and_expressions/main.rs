fn main() {
    let y = {
        let x : i8 = 10;
        x + 10 // no semicolon here
    };

    println!("the value of y is {}", y);

    // statement -> do something, no value returned 
    // expression -> produces a value
}