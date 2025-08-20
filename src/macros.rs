/// Macro `input!`, similar to Python's `input()`.
///
/// Usage:
/// ```no_run
/// use askit::input;
///
/// fn sample() {
///     let name = input!("Enter your name: ");
///     println!("Hello, {name}");
/// }
/// ```
#[macro_export]
macro_rules! input {
    ($msg:expr) => {{ $crate::prompt($msg).get::<String>().unwrap() }};
}
