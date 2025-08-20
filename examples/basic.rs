use askit::input;

fn main() {
    let name: String = input!("Your name: ");
    let age: u8 = input!("Age: ").parse().unwrap();
    println!("Hello, {name} ({age}).");
}
