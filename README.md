# askit

`askit` is a simple and semantic Rust library to handle interactive CLI
prompts, inspired by Python's `input`.\
It provides an ergonomic and type-safe way to ask users for values,
validate them, and display custom messages.

------------------------------------------------------------------------

## Features

-   **`ask!` macro**: The simplest way to get input, panics on error.
    Perfect for quick scripts and demos.\
-   **`input!` macro**: Safe version, returns `Result<T, Error>` so you
    can handle errors gracefully.\
-   **`.force()` helper**: Lets you call `.force()` on an `input!`
    result to panic on error.\
-   **Typed input**: Directly parse inputs into Rust types like `i32`,
    `f64`, `bool`, `String`, and more.\
-   **Validation**: Attach custom validation logic.\
-   **Messages**: Attach messages or hints to guide the user.\
-   **Defaults**: Provide default values if the user presses Enter.

------------------------------------------------------------------------

## Installation

Add the following to your `Cargo.toml`:

``` toml
[dependencies]
askit = "0.1.0"
```

------------------------------------------------------------------------

## Quickstart with `ask!` (recommended)

``` rust
use askit::ask;

fn main() {
    let name: String = ask!("Your name: ");
    let age: u8 = ask!("Age [default=18]: ", default = 18u8, retries = 2);
    println!("Hello, {name} ({age}).");
}
```

------------------------------------------------------------------------

## Usage Variations

### 1. `ask!` macro (recommended, panics on error)

``` rust
use askit::ask;

fn main() {
    let port: u16 = ask!("Port (1..=65535): ", retries = 1);
    println!("Port: {port}");
}
```

### 2. `input!` macro (safe, returns `Result`)

``` rust
use askit::input;

fn main() -> Result<(), askit::Error> {
    let name: String = input!("Name: ")?;
    println!("Name: {name}");
    Ok(())
}
```

### 3. `.force()` helper on `input!` (shortcut)

``` rust
use askit::{input, ForceOk};

fn main() {
    let age: u8 = input!("Age: ").force();
    println!("Age: {age}");
}
```

------------------------------------------------------------------------

## Advanced Usage with `prompt()` Builder

``` rust
use askit::prompt;

fn main() -> Result<(), askit::Error> {
    let pct: f32 = prompt("Percent: ")
        .to::<f32>()
        .default_val(50.0)
        .retries(3)
        .validate(|v| *v >= 0.0 && *v <= 100.0)
        .message("Percent must be between 0 and 100")
        .get()?;
    println!("pct = {pct}");
    Ok(())
}
```

------------------------------------------------------------------------

## Running Examples

``` bash
cargo run --example quickstart
```

------------------------------------------------------------------------

## Testing

``` bash
cargo test
```

------------------------------------------------------------------------

## License

MIT License.
