use anyhow::{Result, bail};
use std::ffi::c_char;

pub fn parse_single_char<T: FromChar>(
    field_value: &str,
    field_name: &str,
    valid_chars: &[char],
    converter: impl Fn(char) -> T,
) -> Result<T> {
    if field_value.len() != 1 {
        bail!(
            "{} must be single character, got: '{}'",
            field_name,
            field_value
        );
    }

    let char_value = field_value.chars().next().unwrap();

    if !valid_chars.contains(&char_value) {
        bail!(
            "Invalid {} character: '{}', expected one of: {:?}",
            field_name,
            char_value,
            valid_chars
        );
    }

    Ok(converter(char_value))
}

pub trait FromChar {
    fn from_char(c: char) -> Self;
}

impl FromChar for c_char {
    fn from_char(c: char) -> Self {
        c as c_char
    }
}

impl FromChar for u8 {
    fn from_char(c: char) -> Self {
        c as u8
    }
}

pub fn parse_action<T: FromChar>(action: &str) -> Result<T> {
    parse_single_char(
        action,
        "action",
        &['A', 'C', 'M', 'R', 'T', 'F', 'N'],
        T::from_char,
    )
}

pub fn parse_side<T: FromChar>(side: &str) -> Result<T> {
    parse_single_char(side, "side", &['A', 'B', 'N'], T::from_char)
}
