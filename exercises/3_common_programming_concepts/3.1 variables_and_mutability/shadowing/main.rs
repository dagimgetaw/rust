fn main() {
    let x = 10;
    let x = x + 10;
    
    {
        let x = x * 2;
        println!("the value of x in the inner scope is {}", x);
    }
    
    let x = x + 10;
    println!("the value of x is {}", x);


    // this is different from mutability 
    // mutability is change the variable while shadowing means create new variable with the same name
    // mut → same variable changes
    //shadowing → new variable replaces old one
}