use std::io::{self, BufRead, Write};
use std::str::FromStr;

/// Errors that `askit` may return.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    /// Failed to parse the input into the requested type.
    Parse {
        ty: &'static str,
        cause: String,
    },
    /// Input was empty and no default was provided.
    EmptyNotAllowed,
    /// All retry attempts were exhausted.
    RetriesExceeded,
    /// Validation failed with user-defined message.
    Validation(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Parse { ty, cause } => write!(f, "Failed to parse as {ty}: {cause}"),
            Error::EmptyNotAllowed => write!(f, "Empty input (no default provided)"),
            Error::RetriesExceeded => write!(f, "Maximum retry attempts exceeded"),
            Error::Validation(msg) => write!(f, "Validation failed: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

/// Untyped builder for reading and parsing CLI input.
pub struct Prompt<'a> {
    pub(crate) message: &'a str,
    pub(crate) default_str: Option<String>,
    pub(crate) retries: usize,
    pub(crate) trim_input: bool,
}

impl<'a> Prompt<'a> {
    /// Create a new prompt with a message.
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            default_str: None,
            retries: 0,
            trim_input: true,
        }
    }

    /// Provide a default value **as string**. If the user hits ENTER with empty input,
    /// `default` will be used and parsed as the target type.
    pub fn default(mut self, default: &str) -> Self {
        self.default_str = Some(default.to_string());
        self
    }

    /// Number of times to retry when parsing fails or input is empty w/o default.
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = retries;
        self
    }

    /// Whether to trim whitespace (default: true).
    pub fn trim(mut self, yes: bool) -> Self {
        self.trim_input = yes;
        self
    }

    /// Convert to a typed builder, enabling `.default_val()` and `.validate()`.
    pub fn to<T>(self) -> TypedPrompt<'a, T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        TypedPrompt {
            base: self,
            default_val: None,
            validator: None,
            validation_msg: None,
        }
    }

    /// Read from **stdin**, parse and return the desired type.
    pub fn get<T>(&self) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let stdin = io::stdin();
        let mut lock = stdin.lock();
        let mut stdout = io::stdout();
        self.get_with(&mut lock, &mut stdout)
    }

    /// Same as `get()`, but allows providing a custom **reader** and **writer**.
    pub fn get_with<T, R, W>(&self, reader: &mut R, writer: &mut W) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
        R: BufRead,
        W: Write,
    {
        let mut attempts_left = self.retries + 1;

        loop {
            // Render message (with default hint, if any)
            {
                let mut msg = String::new();
                msg.push_str(self.message);
                if let Some(def) = &self.default_str {
                    if !self.message.contains('[') && !self.message.contains("(default") {
                        use std::fmt::Write as _;
                        let _ = write!(msg, "[default: {}] ", def);
                    } else {
                        msg.push(' ');
                    }
                }
                writer.write_all(msg.as_bytes())?;
                writer.flush()?;
            }

            let mut line = String::new();
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
                // EOF
                if let Some(def) = &self.default_str {
                    return parse_as::<T>(def);
                }
                return Err(Error::EmptyNotAllowed);
            }

            let mut s = line;
            if self.trim_input {
                s = s.trim().to_string();
            }

            // Empty input handling
            if s.is_empty() {
                if let Some(def) = &self.default_str {
                    match parse_as::<T>(def) {
                        Ok(val) => return Ok(val),
                        Err(e) => return Err(e), // misconfigured default
                    }
                } else {
                    if self.retries == 0 {
                        return Err(Error::EmptyNotAllowed);
                    }
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::RetriesExceeded);
                    }
                    writeln!(writer, "Empty input. Please try again.")?;
                    continue;
                }
            }

            // Try parse
            match parse_as::<T>(&s) {
                Ok(val) => return Ok(val),
                Err(e) => {
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::RetriesExceeded);
                    }
                    writeln!(writer, "{e}")?;
                    continue;
                }
            }
        }
    }
}

/// Typed builder for extras: `.default_val`, `.validate`, `.message`.
pub struct TypedPrompt<'a, T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    pub(crate) base: Prompt<'a>,
    pub(crate) default_val: Option<T>,
    pub(crate) validator: Option<Box<dyn Fn(&T) -> bool + 'static>>,
    pub(crate) validation_msg: Option<String>,
}

impl<'a, T> TypedPrompt<'a, T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    /// Provide a typed default (não precisa parsear).
    pub fn default_val(mut self, val: T) -> Self {
        self.default_val = Some(val);
        self
    }

    /// function with validation `Fn(&T) -> bool`.
    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Box::new(f));
        self
    }

    /// Message shown when `.validate` returns `false`.
    pub fn message(mut self, msg: &str) -> Self {
        self.validation_msg = Some(msg.to_string());
        self
    }

    /// Number of times to retry (aplica no `Prompt` base).
    pub fn retries(mut self, retries: usize) -> Self {
        self.base.retries = retries;
        self
    }

    /// Whether to trim whitespace (default: true).
    pub fn trim(mut self, yes: bool) -> Self {
        self.base.trim_input = yes;
        self
    }

    /// read and return `T` using stdin/stdout.
    pub fn get(self) -> Result<T, Error> {
        let mut reader = io::stdin().lock();
        let mut writer = io::stdout();
        self.get_with_io(&mut reader, &mut writer)
    }

    pub fn get_with_io<R, W>(self, reader: &mut R, writer: &mut W) -> Result<T, Error>
    where
        R: BufRead,
        W: Write,
    {
        let mut attempts_left = self.base.retries + 1;
        let mut default_val = self.default_val;
        let validator = self.validator;
        let validation_msg = self.validation_msg;
        let base = self.base;

        loop {
            // Render message (com dica de default)
            {
                let mut msg = String::new();
                msg.push_str(base.message);
                if default_val.is_some() {
                    if !base.message.contains('[') && !base.message.contains("(default") {
                        use std::fmt::Write as _;
                        let _ = write!(msg, "[default set] ");
                    } else {
                        msg.push(' ');
                    }
                } else if let Some(def) = &base.default_str {
                    if !base.message.contains('[') && !base.message.contains("(default") {
                        use std::fmt::Write as _;
                        let _ = write!(msg, "[default: {}] ", def);
                    } else {
                        msg.push(' ');
                    }
                }
                writer.write_all(msg.as_bytes())?;
                writer.flush()?;
            }

            let mut line = String::new();
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
                if let Some(v) = default_val.take() {
                    return Ok(v);
                } else if let Some(def) = &base.default_str {
                    return parse_as::<T>(def);
                } else {
                    return Err(Error::EmptyNotAllowed);
                }
            }

            let mut s = line;
            if base.trim_input {
                s = s.trim().to_string();
            }

            if s.is_empty() {
                if let Some(v) = default_val.take() {
                    return Ok(v);
                } else if let Some(def) = &base.default_str {
                    match parse_as::<T>(def) {
                        Ok(val) => return Ok(val),
                        Err(e) => return Err(e),
                    }
                } else {
                    if base.retries == 0 {
                        return Err(Error::EmptyNotAllowed);
                    }
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::RetriesExceeded);
                    }
                    writeln!(writer, "Empty input. Please try again.")?;
                    continue;
                }
            }

            // Parse
            let val = match parse_as::<T>(&s) {
                Ok(v) => v,
                Err(e) => {
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::RetriesExceeded);
                    }
                    writeln!(writer, "{e}")?;
                    continue;
                }
            };

            // Validação
            if let Some(vf) = &validator {
                if !vf(&val) {
                    let msg = validation_msg.clone().unwrap_or_else(|| "Invalid value".to_string());
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::Validation(msg));
                    }
                    writeln!(writer, "{msg}")?;
                    continue;
                }
            }

            return Ok(val);
        }
    }
}

/// parse helper
fn parse_as<T>(s: &str) -> Result<T, Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    T::from_str(s).map_err(|e| Error::Parse {
        ty: std::any::type_name::<T>(),
        cause: e.to_string(),
    })
}

/// Entry-point function to create a `Prompt`.
pub fn prompt(message: &str) -> Prompt<'_> {
    Prompt::new(message)
}
