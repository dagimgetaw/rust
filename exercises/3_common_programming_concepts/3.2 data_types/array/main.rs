fn main() {
    let nums : [i8 ; 5] = [1, 2, 3, 4, 5];

    println!("the value of the first number in the array {}", nums[0]);
    println!("the value of the second number in the array {}", nums[1]);
    println!("the value of the third number in the array {}", nums[2]);
    println!("the value of the fourth number in the array {}", nums[3]);
    println!("the value of the fivth number in the array {}", nums[4]);

    let _months : [&str ; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    // '_' tells the compiler to execute even if i didn't use the variable
}