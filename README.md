# askit
[![Crates.io](https://img.shields.io/crates/v/askit.svg)](https://crates.io/crates/askit)
[![Documentation](https://docs.rs/askit/badge.svg)](https://docs.rs/askit)

`askit` is a simple and semantic Rust library to handle interactive CLI
prompts, inspired by Python’s `input`.

It provides a **pythonic `input!` macro** (always returns `String`) and
an advanced **`Prompt` builder API** for developers who need retries,
defaults, type safety, and validation.

---

## Features

- **`input!` macro**: Python-like, minimal, always returns `String`.
- **`Prompt` builder**: Advanced API for typed input, defaults, retries, and validation.
- **Type-safe parsing**: With `Prompt`, you can directly parse into Rust types.
- **Validation**: Attach custom validation logic to user input.
- **Default values**: Provide defaults if the user presses ENTER.
- **Retries**: Allow multiple attempts on invalid input.

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
askit = "0.2.0"
```

---

## Quickstart with input!

```rust
use askit::input;

fn main() {
    let name = input!("Your name: ");
    println!("Hello, {name}");

    // Parsing is your responsibility (just like Python’s int(input()))
    let age: u8 = input!("Age: ").parse().unwrap_or(18);
    println!("Age: {age}");
}
```

---

## input! macro

- Always returns a `String`.
- No retries, no defaults, no validation — keep it minimal and Pythonic.
- If you need anything beyond that, use `Prompt`.

Examples:

```rust
use askit::input;

fn main() {
    let city = input!("City: ");
    println!("City = {city}");

    let number: i32 = input!("Number: ").parse().unwrap();
    println!("Number = {number}");
}
```

---

## Advanced usage with Prompt

The `Prompt` builder gives you more control:

```rust
use askit::prompt;

fn main() -> Result<(), askit::Error> {
    let threads: usize = prompt("Threads: ")
        .to::<usize>()                 // type-safe parsing
        .default_val(4)                // default if empty input
        .retries(2)                    // allow multiple attempts
        .validate(|v| *v > 0)          // custom validation
        .message("Threads must be > 0")// custom error message
        .get()?;                       // final input

    println!("threads = {threads}");
    Ok(())
}
```

### What Prompt can do

- `.default("str")` → provide a string default.
- `.to::<T>()` → switch to typed input (`i32`, `f64`, custom types, etc).
- `.default_val(T)` → set a typed default.
- `.retries(n)` → allow multiple attempts.
- `.trim(true/false)` → control whitespace trimming.
- `.validate(|v| ...)` → attach a custom validator.
- `.message("...")` → custom validation error message.
- `.get()` → read input and return `Result<T, Error>`.

---

## Error Handling

All fallible operations return `Result<T, askit::Error>`.

Errors include:

- `Io` → I/O error when reading/writing.
- `Parse` → Failed to parse into the requested type.
- `EmptyNotAllowed` → Input was empty and no default was provided.
- `RetriesExceeded` → Too many failed attempts.
- `Validation` → Custom validation failed.

---

## Running Examples

```bash
cargo run --example input_showcases
```

---

## Testing

```bash
cargo test
```

---

## License

MIT License.
