use core::panic;
use nanoid::nanoid;
use serde::Serialize;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use tauri::regex::Regex;

pub fn random_id() -> String {
    const NANOID_ALPHABET: &'static str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let chars: Vec<char> = NANOID_ALPHABET.chars().collect();
    nanoid!(21, &chars)
}

pub fn read_json_file(file_path: String) -> Result<serde_json::Value, Error> {
    let file_contents = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Err(e),
            _ => panic!("Unknown error"),
        },
    };

    let data: serde_json::Value =
        serde_json::from_str(&file_contents).expect("Unable to parse JSON");
    Ok(data)
}

pub fn write_json_file<T: Serialize>(file_path: String, input: &T) {
    let object_str = serde_json::to_string(input).expect("Unable to write to JSON file");
    fs::write(&file_path, object_str).expect("Unable to write to file")
}

pub fn capture_values(re_string: &str, val_string: String) -> Vec<String> {
    let re = Regex::new(&re_string).expect("Invalid regex");
    let caps = re
        .captures(&val_string)
        .expect(&format!("Cannot capture values from string: {val_string}"));

    let result = caps
        .iter()
        .filter_map(|x| x)
        .map(|x| {
            let result = x.as_str().to_owned();
            result
        })
        .collect::<Vec<String>>();

    result
}
