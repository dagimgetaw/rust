// methods are like a function that belong to a specific type (like a struct).
struct Dog {
    name: String,
    age: u32,
}

impl Dog {
    // Method to make the dog bark
    fn bark(&self) {
        println!("{} says: Woof!", self.name);
    }
    
    // Method to check if dog is old
    fn is_old(&self) -> bool {
        self.age > 10
    }
    
    // Method to have a birthday
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("{} is now {} years old!", self.name, self.age);
    }
}

fn main() {
    let mut my_dog = Dog {
        name: String::from("Buddy"),
        age: 5,
    };
    
    my_dog.bark();           // Prints: Buddy says: Woof!
    println!("Old? {}", my_dog.is_old());  // Prints: Old? false
    my_dog.have_birthday();  // Prints: Buddy is now 6 years old!
}