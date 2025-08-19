use askit::input;
use std::str::FromStr;

/// Example of a custom type: "x,y" -> Point { x, y }
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').map(|t| t.trim()).collect();
        if parts.len() != 2 {
            return Err("expected format: x,y".into());
        }
        let x: i32 = parts[0].parse().map_err(|e| format!("invalid x: {e}"))?;
        let y: i32 = parts[1].parse().map_err(|e| format!("invalid y: {e}"))?;
        Ok(Point { x, y })
    }
}

fn main() -> Result<(), askit::Error> {
    println!("=== askit::input! showcase ===");
    println!("(Tips: ENTER uses the default if defined; 'retries' allows extra attempts)");

    // 1) Simple string (no default, no retries)
    let name: String = input!("Your name: ")?;
    println!("> name = {name}");

    // 2) String with typed default (use String::from(...) for string defaults)
    let username: String = input!(
        "Username [default=guest]: ",
        default = String::from("guest")
    )?;
    println!("> username = {username}");

    // 3) Integer (u8) with default AND retries
    let age: u8 = input!(
        "Age [default=18] (retries=2): ",
        default = 18u8,
        retries = 2
    )?;
    println!("> age = {age}");

    // 4) Integer (u16) with retries (no default)
    let port: u16 = input!("Port (1..=65535) (retries=2): ", retries = 2)?;
    println!("> port = {port}");

    // 5) Integer (usize) with default (e.g., number of threads)
    let threads: usize = input!("Threads [default=4]: ", default = 4usize)?;
    println!("> threads = {threads}");

    // 6) Floating-point (f32) with default + retries
    let price: f32 = input!(
        "Price [default=9.99] (retries=1): ",
        default = 9.99f32,
        retries = 1
    )?;
    println!("> price = {price}");

    // 7) Floating-point (f64) without default (retry on parse error)
    let tax: f64 = input!("Tax (e.g., 12.5): ", retries = 1)?;
    println!("> tax = {tax}");

    // 8) Boolean with default (true/false)
    let confirm: bool = input!("Continue? (true/false) [default=true]: ", default = true)?;
    println!("> confirm = {confirm}");

    // 9) Custom type (Point) with retries
    //    Example of valid input: "10, 20"
    let p: Point = input!("Point (format x,y) (retries=2): ", retries = 2)?;
    println!("> point = {:?}", p);

    println!("=== done ===");
    Ok(())
}
