pub mod error;
pub mod parse;
pub mod prompt;
pub mod typed_prompt;
pub mod validator;

pub use error::Error;
pub use prompt::Prompt;
pub use typed_prompt::TypedPrompt;
pub use validator::Validator;

pub fn prompt(message: &str) -> Prompt<'_> {
    Prompt::new(message)
}
