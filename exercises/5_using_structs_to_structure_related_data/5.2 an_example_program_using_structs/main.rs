// example to understand struct 

struct Rectangle {
    width : i8,
    height : i8
}

fn main() {
    let rect = Rectangle {
        width: 3,
        height: 2,
    };
    let result = area(&rect);
    println!("result are {result}");
}

fn area(dimension : &Rectangle) -> i8 {
    dimension.width * dimension.height
}