use core::panic;
use std::fs;

use crate::parse;

pub fn get_json_data(path: &str) -> i18n_locale_lint_ast::value::Value {
    let data = fs::read_to_string(path);
    let content = if let Ok(content) = data {
        content
    } else {
        panic!("failed to parse");
    };

    parse::parse(content)
}
