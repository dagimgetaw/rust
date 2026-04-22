fn main() {
    let tup1 : (i8, i16, f32) = (100, 200, 3.14);

    println!("tuple one value is {}", tup1.0);
    println!("tuple two value is {}", tup1.1);
    println!("tuple three value is {}", tup1.2);


    // tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // tuples have a fixed length: Once declared, they cannot grow or shrink in size.

    let tup2 = (10, 20, 30);
    let (x, y, z) = tup2;

    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
    println!("the value of z is {}", z);
    
}