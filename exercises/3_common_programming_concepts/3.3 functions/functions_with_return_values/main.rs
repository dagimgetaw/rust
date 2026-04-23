fn main() {
    let x : i8 = 10;
    let y : i8 = 5;
    let sum : i8 = add(x, y);
    
    println!("the sum of {} and {} are {}", x, y, sum);
}

fn add(x : i8, y: i8) -> i8 {
    return x + y;
}
