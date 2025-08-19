use askit::{Error, prompt};
use std::io::{Cursor, sink};

#[test]
fn parse_integer_ok() {
    let input = b"42\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();
    let p = prompt("Num: ");
    let n: i32 = p.get_with(&mut reader, &mut writer).unwrap();
    assert_eq!(n, 42);
}

#[test]
fn empty_with_default_uses_default() {
    let input = b"\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();
    let p = prompt("Port: ").default("5432");
    let port: u16 = p.get_with(&mut reader, &mut writer).unwrap();
    assert_eq!(port, 5432);
}

#[test]
fn parse_error_then_retry_then_ok() {
    let input = b"oops\n123\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();
    let p = prompt("Age: ").retries(1);
    let age: i32 = p.get_with(&mut reader, &mut writer).unwrap();
    assert_eq!(age, 123);
}

#[test]
fn retries_exceeded() {
    let input = b"oops\nnope\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();
    let p = prompt("N: ").retries(1);
    let res: Result<i32, Error> = p.get_with(&mut reader, &mut writer);
    assert!(matches!(res, Err(Error::RetriesExceeded)));
}

#[test]
fn empty_without_default_fails() {
    let input = b"\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();
    let p = prompt("Name: ");
    let res: Result<String, Error> = p.get_with(&mut reader, &mut writer);
    assert!(matches!(
        res,
        Err(Error::RetriesExceeded) | Err(Error::EmptyNotAllowed)
    ));
}
