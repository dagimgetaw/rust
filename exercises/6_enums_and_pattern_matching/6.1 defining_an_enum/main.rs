// an enum (enumeration) is a way to say "a value can be one of several possible options.
// think of it like a multiple-choice question - only one answer can be selected.

// real world analogy
// imagine a traffic light. tt can only be one of three states at any time: Red, Yellow, or Green.
// it can't be red AND green simultaneously. This is exactly what enums represent!

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let current_light = TrafficLight::Red;
    let next_light = TrafficLight::Green;
    
    check_light(current_light);
    check_light(next_light);
}

fn check_light(light: TrafficLight) {
    // We'll learn how to check which variant it is later
    println!("Light is working!");
}

enum Message {
    Quit,                              // No data
    Move { x: i32, y: i32 },          // Named fields (like a struct)
    Write(String),                     // Single String
    ChangeColor(i32, i32, i32),       // Three i32 values
}

fn main() {
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    // All these different message types are the SAME type: Message
    // This means we can put them all in the same collection!
    let messages = vec![msg1, msg2, msg3, msg4];
}