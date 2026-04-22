fn main() {
    let x : i8 = 10;
    println!("the value of x is {}", x);

    x = 20;
    println!("the updated value of x is {}", x); // cannot assign twice to immutable variable `x`

    // variable as a default are immutable which means u can't update the value after they created
    // so inorder to make the variable updateable or mutable we must define the varible as mutable

    let mut x : i8 = 10; // the keyword "mut" makes the variable mutable
    println!("the value of x is {}", x);
    
    x = 20;
    println!("the updated value of x is {}", x);

    // output
    // the value of x is 10
    // the updated value of x is 20
}