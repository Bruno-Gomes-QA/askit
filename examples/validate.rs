use askit::prompt;

fn main() -> Result<(), askit::Error> {
    let port: u32 = prompt("Port: ")
        .to()
        .retries(2)
        .validate(|p| (1..=65535).contains(p))
        .message("Port must be in 1..=65535")
        .get()?;

    println!("Using port {port}");
    Ok(())
}
