use askit::ask;

fn main() {
    let name: String = ask!("Your name: ");
    let age: u8 = ask!(
        "Age [default=18] (retries=2): ",
        default = 18u8,
        retries = 2
    );
    println!("Hello, {name} ({age}).");
}
