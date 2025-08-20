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

fn main() {
    println!("=== askit::input! showcase ===");
    println!("(Tip: ENTER without typing returns an empty string)");

    // 1) Simple string
    let name = input!("Your name: ");
    println!("> name = {name}");

    // 2) String with default (handled in code, not macro)
    let username = {
        let raw = input!("Username (default = guest): ");
        if raw.is_empty() {
            "guest".to_string()
        } else {
            raw
        }
    };
    println!("> username = {username}");

    // 3) Integer: manual parse + default
    let age = input!("Age: ").parse::<u8>().unwrap_or(18);
    println!("> age = {age}");

    // 4) Integer with manual retry loop
    let port = loop {
        let raw = input!("Port (1..=65535): ");
        match raw.parse::<u16>() {
            Ok(p) => break p,
            Err(_) => {
                println!("Invalid port, please try again");
                continue;
            }
        }
    };
    println!("> port = {port}");

    // 5) Integer with default using unwrap_or
    let threads = input!("Threads: ").parse::<usize>().unwrap_or(4);
    println!("> threads = {threads}");

    // 6) Float (f32) with default
    let price = input!("Price: ").parse::<f32>().unwrap_or(9.99);
    println!("> price = {price}");

    // 7) Float (f64) with retry loop
    let tax = loop {
        let raw = input!("Tax (e.g., 12.5): ");
        match raw.parse::<f64>() {
            Ok(t) => break t,
            Err(_) => {
                println!("Invalid number, try again");
                continue;
            }
        }
    };
    println!("> tax = {tax}");

    // 8) Boolean with default
    let confirm = input!("Continue? (true/false): ")
        .parse::<bool>()
        .unwrap_or(true);
    println!("> confirm = {confirm}");

    // 9) Custom type (Point)
    //    Example input: "10,20"
    let p: Point = input!("Point (format x,y): ")
        .parse()
        .expect("invalid point format");
    println!("> point = {:?}", p);

    println!("=== done ===");
}
