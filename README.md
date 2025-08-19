# askit

`askit` is a simple and semantic Rust library to handle interactive CLI prompts, inspired by Python's `input`.  
It provides an ergonomic and type-safe way to ask users for values, validate them, and display custom messages.

---

## Features

- **`input!` macro**: Ask the user for input with minimal boilerplate.  
- **Typed input**: Directly parse inputs into Rust types like `i32`, `f64`, `bool`, `String`, and more.  
- **Validation**: Attach custom validation logic.  
- **Messages**: Attach messages or hints to guide the user.  
- **Defaults**: Provide default values if the user presses Enter.  

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
askit = "0.1.0"
```

---

## Quickstart

```rust
use askit::input;

fn main() {
    // Basic input as string
    let name: String = input!("What is your name? ");
    println!("Hello, {}!", name);

    // Input as integer
    let age: i32 = input!("How old are you? ");
    println!("You are {} years old.", age);

    // With default value
    let country: String = askit::Input::new("Your country: ")
        .default("Brazil".to_string())
        .get();
    println!("Country: {}", country);

    // With validation
    let age: i32 = askit::Input::new("Age (must be > 18): ")
        .validate(|&n| if n > 18 { Ok(()) } else { Err("Age must be > 18".into()) })
        .get();
    println!("Validated Age: {}", age);
}
```

---

## Usage Examples

### 1. Basic `input!` macro

```rust
use askit::input;

fn main() {
    let name: String = input!("Enter your name: ");
    let age: i32 = input!("Enter your age: ");
    println!("{} is {} years old.", name, age);
}
```

### 2. With default values

```rust
use askit::Input;

fn main() {
    let language: String = Input::new("Favorite language: ")
        .default("Rust".to_string())
        .get();
    println!("Favorite language: {}", language);
}
```

### 3. With validation

```rust
use askit::Input;

fn main() {
    let score: i32 = Input::new("Score (0-100): ")
        .validate(|&s| {
            if (0..=100).contains(&s) {
                Ok(())
            } else {
                Err("Score must be between 0 and 100".into())
            }
        })
        .get();
    println!("Validated score: {}", score);
}
```

---

## Running Examples

Run the examples included in the repository:

```bash
cargo run --example basic
cargo run --example validation
```

---

## Testing

Run the test suite:

```bash
cargo test
```

---

## Development Guide

### Branching Strategy

- **`main`**: Always production-ready.  
- **`develop`**: Integration branch for new features.  
- **`feature/*`**: Each feature should have its own branch.  
- **`hotfix/*`**: For urgent fixes to production.  

### Commit Convention

Follow the [Conventional Commits](https://www.conventionalcommits.org/) standard:

- `feat:` – New feature.  
- `fix:` – Bug fix.  
- `docs:` – Documentation changes.  
- `style:` – Code style changes (formatting, etc).  
- `refactor:` – Code refactoring.  
- `test:` – Adding or fixing tests.  
- `chore:` – Other changes (build tools, CI, etc).  

Example:

```
feat(input): add default typed values
fix(validate): correct error message display
```

---

## License

MIT License.
