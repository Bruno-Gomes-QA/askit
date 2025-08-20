use std::io::{self, BufRead, Write};
use std::str::FromStr;

use crate::prompt_mod::{error::Error, parse::parse_as, typed_prompt::TypedPrompt};

pub struct Prompt<'a> {
    pub(crate) message: &'a str,
    pub(crate) default_str: Option<String>,
    pub(crate) retries: usize,
    pub(crate) trim_input: bool,
}

impl<'a> Prompt<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            message,
            default_str: None,
            retries: 0,
            trim_input: true,
        }
    }

    pub fn default(mut self, default: &str) -> Self {
        self.default_str = Some(default.to_string());
        self
    }

    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = retries;
        self
    }

    pub fn trim(mut self, yes: bool) -> Self {
        self.trim_input = yes;
        self
    }

    pub fn to<T>(self) -> TypedPrompt<'a, T>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        TypedPrompt::new(self)
    }

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

    pub fn get_with<T, R, W>(&self, reader: &mut R, writer: &mut W) -> Result<T, Error>
    where
        T: FromStr,
        T::Err: std::fmt::Display,
        R: BufRead,
        W: Write,
    {
        let mut attempts_left = self.retries + 1;

        loop {
            // render message
            {
                let mut msg = String::new();
                msg.push_str(self.message);
                if let Some(def) = &self.default_str {
                    if !self.message.contains('[') && !self.message.contains("(default") {
                        use std::fmt::Write as _;
                        let _ = write!(msg, " [default: {}]", def);
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
                } else {
                    return parse_as::<T>(""); // empty input (Python-like)
                }
            }

            let mut s = line;
            if self.trim_input {
                s = s.trim().to_string();
            }

            // empty input
            if s.is_empty() {
                if let Some(def) = &self.default_str {
                    return parse_as::<T>(def);
                } else {
                    return parse_as::<T>(""); // no retries, just ""
                }
            }

            // try parse
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
