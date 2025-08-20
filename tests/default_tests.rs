use askit::prompt_mod::prompt;
use std::io::{Cursor, sink};

#[test]
fn typed_default_used_on_empty() {
    let input = b"\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();

    let port: u16 = prompt("Port: ")
        .to()
        .default_val(5432)
        .get_with_io(&mut reader, &mut writer)
        .unwrap();

    assert_eq!(port, 5432);
}

#[test]
fn typed_default_has_priority_over_string_default() {
    let input = b"\n";
    let mut reader = Cursor::new(input);
    let mut writer = sink();

    let port: u16 = prompt("Port: ")
        .default("1111")
        .to()
        .default_val(2222)
        .get_with_io(&mut reader, &mut writer)
        .unwrap();

    assert_eq!(port, 2222);
}
