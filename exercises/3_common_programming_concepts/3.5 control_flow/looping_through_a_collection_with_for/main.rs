fn main() {
    let arr : [usize ; 5] = [1, 2, 3, 4, 5];
    let mut index : usize = 0;
    
    while index < 5 {
        println!("arr at index {} is {}", index, arr[index]);
        index += 1;
    }
    
    for num in arr {
        println!("{}", num);
    }
    
    for i in 6..=10 {
        println!("{}", i);
    }
}