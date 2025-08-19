//! askit: a simple and ergonomic CLI input library.
//!
//! Quickstart (com Result):
//! ```no_run
//! use askit::prompt;
//!
//! fn main() -> Result<(), askit::Error> {
//!     let name: String = prompt("Name: ").get()?;
//!     let age: u8 = prompt("Age [18]: ").default("18").retries(2).get()?;
//!     println!("Hello, {} ({}).", name, age);
//!     Ok(())
//! }
//! ```
//!

mod prompt;
mod macros;

pub use prompt::{prompt, Error, Prompt, TypedPrompt};

/// Helper `Result<T, Error>`.
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
