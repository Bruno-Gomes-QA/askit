/// Sugar macro similar to Python's `input()`.
///
/// Usage:
/// ```ignore
/// let name: String = input!("Name: ");
/// let tries: u8 = input!("Retries (default 3): ", default = 3);
/// let port: u16 = input!("Port: ", retries = 2);
/// let pct: f32 = input!("Percent [50.0]: ", default = 50.0, retries = 3);
/// ```
#[macro_export]
macro_rules! input {
    // msg only
    ($msg:expr) => {{ $crate::prompt($msg).get() }};

    // msg, retries = N
    ($msg:expr, retries = $r:expr) => {{ $crate::prompt($msg).retries($r).get() }};

    // msg, default = "str" | number (type with .to::<T>().default_val)
    ($msg:expr, default = $d:expr) => {{ $crate::prompt($msg).to().default_val($d).get() }};

    // msg, default = X, retries = N (1)
    ($msg:expr, default = $d:expr, retries = $r:expr) => {{ $crate::prompt($msg).to().default_val($d).retries($r).get() }};

    // msg, retries = N, default = X (2)
    ($msg:expr, retries = $r:expr, default = $d:expr) => {{ $crate::prompt($msg).to().default_val($d).retries($r).get() }};
}

#[macro_export]
macro_rules! ask {
    ($($tt:tt)*) => {
        $crate::input!($($tt)*).unwrap()
    };
}
