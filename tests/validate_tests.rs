use askit::{Error, prompt_mod::prompt};
use std::io::{Cursor, sink};

#[test]
fn validate_fail_then_ok_with_retries() {
    let input = b"70000\n65535\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();

    let port: u16 = prompt("Port: ")
        .to()
        .retries(1) // falha uma vez, depois acerta
        .validate(|p| (1..=65535).contains(p))
        .message("Port must be 1..=65535")
        .get_with_io(&mut reader, &mut writer)
        .unwrap();

    assert_eq!(port, 65535);
}

#[test]
fn validate_fail_no_retries_returns_validation_error() {
    let input = b"0\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();

    let res: Result<u16, Error> = prompt("Port: ")
        .to()
        .retries(0)
        .validate(|p| (1..=65535).contains(p))
        .message("Invalid port")
        .get_with_io(&mut reader, &mut writer);

    match res {
        Err(Error::Validation(msg)) => assert_eq!(msg, "Invalid port"),
        other => panic!("expected Validation error, got {:?}", other),
    }
}
