mod diff;
mod display;
mod files;
mod is_same_type;
mod option;

use std::env;
use std::fs;
use std::process::exit;
use serde_json::Value;

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

fn check(file_paths: &Vec<&str>) -> bool {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = args[1].as_str();

    let option = option::Option::from_cli(&args);
    let _ = option::INSTANCE.set(option);

    let mut ok = true;
    match files::get_file_groups(pattern) {
        Ok(file_groups) => {
            for group in file_groups {
                ok &= check(&group.1.iter().map(|p| p.as_str()).collect());
            }
        }
        Err(e) => {
            println!("{:?}", e);
            ok = false;
        }
    }

    if !ok {
        exit(1);
    }
}
