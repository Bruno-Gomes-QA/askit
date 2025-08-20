use std::io::{self, BufRead, Write};
use std::str::FromStr;

use crate::prompt_mod::{error::Error, parse::parse_as, prompt::Prompt, validator::Validator};

pub struct TypedPrompt<'a, T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    pub(crate) base: Prompt<'a>,
    pub(crate) default_val: Option<T>,
    pub(crate) validator: Option<Validator<T>>,
    pub(crate) validation_msg: Option<String>,
}

impl<'a, T> TypedPrompt<'a, T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    pub(crate) fn new(base: Prompt<'a>) -> Self {
        Self {
            base,
            default_val: None,
            validator: None,
            validation_msg: None,
        }
    }

    pub fn default_val(mut self, val: T) -> Self {
        self.default_val = Some(val);
        self
    }

    pub fn validate<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.validator = Some(Box::new(f));
        self
    }

    pub fn message(mut self, msg: &str) -> Self {
        self.validation_msg = Some(msg.to_string());
        self
    }

    pub fn retries(mut self, retries: usize) -> Self {
        self.base.retries = retries;
        self
    }

    pub fn trim(mut self, yes: bool) -> Self {
        self.base.trim_input = yes;
        self
    }

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
                    attempts_left -= 1;
                    if attempts_left == 0 {
                        return Err(Error::RetriesExceeded);
                    }
                    writeln!(writer, "Empty input. Please try again.")?;
                    continue;
                }
            }

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

            if let Some(vf) = &validator
                && !vf(&val)
            {
                let msg = validation_msg
                    .clone()
                    .unwrap_or_else(|| "Invalid value".to_string());
                attempts_left -= 1;
                if attempts_left == 0 {
                    return Err(Error::Validation(msg));
                }
                writeln!(writer, "{msg}")?;
                continue;
            }

            return Ok(val);
        }
    }
}
