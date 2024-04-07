use i18n_locale_lint_ast::value::Value;
use std::{ffi::OsStr, path::Path};

use crate::option;

fn read_json_file(path: &str) -> Value {
    let data = match Path::new(path).extension().and_then(OsStr::to_str) {
        Some("json") => i18n_locale_lint_json::get_json_data(path),
        Some("yaml") | Some("yml") => i18n_locale_lint_yaml::get_yaml_data(path),
        _ => panic!(),
    };

    if option::INSTANCE.get().unwrap().skip_top_level {
        return data.skip_top_level();
    }

    data
}

pub fn check(file_paths: &Vec<&str>) -> i32 {
    if file_paths.len() <= 1 {
        return 0;
    }

    if !option::INSTANCE.get().unwrap().silent {
        println!("comparing:");
        for path in file_paths {
            println!("- {}", path);
        }
        println!();
    }

    let mut status_code = 0;
    let base_file_path = &file_paths[0];
    let base_data = read_json_file(base_file_path);
    for &target_file_path in file_paths.iter().skip(1) {
        let target_data = read_json_file(target_file_path);
        let result = base_data.compare(&mut vec![], &target_data);
        if let Some(diff) = result {
            diff.display(base_file_path, target_file_path);
            status_code = 1;
        }
    }

    if !option::INSTANCE.get().unwrap().silent {
        if status_code == 0 {
            println!("Ok!");
        }
        println!();
    }

    status_code
}
