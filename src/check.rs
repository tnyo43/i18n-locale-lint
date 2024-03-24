use std::fs;
use serde_json::Value;

use crate::{display, is_same_type, option};

fn read_json_file(path: &str) -> Value {
    let data = fs::read_to_string(path);
    let content = if let Ok(content) = data {
        content
    } else {
        "failed to parse".to_string()
    };

    let parsed_data: Value = serde_json::from_str(&content).unwrap();

    parsed_data
}

pub fn check(file_paths: &Vec<&str>) -> bool {
    if file_paths.len() <= 1 {
        return true;
    }

    if !option::INSTANCE.get().unwrap().silent {
        println!("comparing:");
        for path in file_paths {
            println!("- {}", path);
        }
        println!();
    }

    let mut success = true;
    let base_file_path = &file_paths[0];
    let base_data = read_json_file(base_file_path);
    for &target_file_path in file_paths.iter().skip(1) {
        let target_data = read_json_file(target_file_path);
        let result = is_same_type::is_same_type(&base_data, &target_data, &mut vec![]);
        if let Some(diff) = result {
            display::display_diff(&diff, base_file_path, target_file_path);
            success = false;
        }
    }

    if !option::INSTANCE.get().unwrap().silent {
        if success {
            println!("Ok!");
        }
        println!();
    }

    success
}
