use core::panic;
use std::fs;
use serde_json::{self};

use crate::convert;

fn read_json_file(path: &str) -> serde_json::Value {
    let data = fs::read_to_string(path);
    let content = if let Ok(content) = data {
        content
    } else {
        panic!("failed to parse");
    };

    serde_json::from_str(&content).unwrap()
}

pub fn get_json_data(path: &str) -> i18n_locale_lint_ast::value::Value {
    convert::convert(&read_json_file(path))
}
