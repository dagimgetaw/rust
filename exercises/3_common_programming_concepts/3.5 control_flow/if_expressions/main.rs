fn main() {
    let x : i8 = 10;
    if x < 10 {
        println!("the condition is true");
    } else {
        println!("the condition is false")
    }
    
    let y : i8 = 6;
    if y % 4 == 0 {
        println!("{} is divisible by 4", y);
    } else if y % 3 == 0 {
        println!("{} is divisible by 3", y);
    } else if y % 2 == 0 {
        println!("{} is divisible by 2", y);
    } else {
        println!("{} isn't divisible by 4, 3, and 2", y);
    }

    let condition : bool = true;
    let z : i8 = if condition {10} else {20};
    println!("z is {}", z)
}