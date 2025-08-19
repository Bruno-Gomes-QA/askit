//! askit: a simple and semantic CLI input library.
//!
//! Quickstart:
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

mod prompt;
mod macros;

pub use prompt::{prompt, Error, Prompt, TypedPrompt};