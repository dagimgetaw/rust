fn main() {
    let x : i8 = 10;
    let y : i8 = 2;
    
    let sum = x + y;
    println!("the sum of {} and {} are {}", x, y, sum);
    
    let sub = x - y;
    println!("the subtraction of {} and {} are {}", x, y, sub);
    
    let mul = x * y;
    println!("the multiplication of {} and {} are {}", x, y, mul);
    
    let div = x / y;
    println!("the division of {} and {} are {}", x, y, div);
    
    let rem = x % y;
    println!("the remainder of {} and {} are {}", x, y, rem);
}