use std::io;

fn main() {
    println!("---------------- GUESS THE NUMBER ---------------- ");
    println!("Please enter ur number: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("You guessed = {}", guess);
}
