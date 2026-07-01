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