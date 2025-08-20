//! askit: a simple and ergonomic CLI input library.
//!
//! Quickstart com `Result`:
//! ```no_run
//! use askit::prompt;
//!
//! fn sample() -> Result<(), askit::Error> {
//!     let name: String = prompt("Name: ").get()?;
//!     let age: u8 = prompt("Age [18]: ").default("18").retries(2).get()?;
//!     println!("Hello, {} ({}).", name, age);
//!     Ok(())
//! }
//! ```
//!
//! Quickstart com macro `input!` (return only String):
//! ```no_run
//! use askit::input;
//!
//! let name = input!("Name: ");
//! println!("Hello, {name}");
//! ```

mod macros;
pub mod prompt_mod;

pub use prompt_mod::{Error, Prompt, TypedPrompt, Validator, prompt};

/// Helper `Result<T, Error>` → forçar unwrap com panic elegante.
pub trait ForceOk<T> {
    fn force(self) -> T;
}

impl<T> ForceOk<T> for Result<T, Error> {
    fn force(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => panic!("askit error: {e}"),
        }
    }
}
