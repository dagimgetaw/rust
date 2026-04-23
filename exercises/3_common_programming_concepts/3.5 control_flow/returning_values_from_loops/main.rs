fn main() {
    let mut counter : i8 = 0;
    let value : i8 = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("counter is {} and value is {}", counter, value);
}