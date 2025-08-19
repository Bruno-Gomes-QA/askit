use askit::prompt;

fn main() -> Result<(), askit::Error> {
    let name: String = prompt("Name: ").get()?;
    let age: u8 = prompt("Age: ").default("18").retries(2).get()?;
    println!("Hello, {name} ({age}).");
    Ok(())
}
