fn main() {
    println!("main function");
    another_function(10);
    time(5, 'h');
}

fn another_function(x : i8) {
    println!("another function with parameter {}", x);
}

fn time(h : i8, c : char) {
    println!("the time is {}{}", h, c);
}