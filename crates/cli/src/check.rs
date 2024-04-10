use i18n_locale_lint_ast::value::Value;
use std::{ffi::OsStr, fs, path::Path};

use crate::{error::CliError, option};

fn read_json_file(path: &str) -> Result<Value, CliError> {
    let get_data = match Path::new(path).extension().and_then(OsStr::to_str) {
        Some("json") => i18n_locale_lint_json::parse,
        Some("yaml") | Some("yml") => i18n_locale_lint_yaml::parse,
        v => return Err(CliError::UnknownExtension(path, v)),
    };

    let content = fs::read_to_string(path);
    let data = match content {
        Ok(content) => get_data(&content),
        Err(e) => return Err(CliError::FileReadError(e)),
    };

    if option::INSTANCE.get().unwrap().skip_top_level {
        return Ok(data.skip_top_level());
    }

    Ok(data)
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
    let base_data = match read_json_file(base_file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    };

    for &target_file_path in file_paths.iter().skip(1) {
        let target_data = match read_json_file(target_file_path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{}", e);
                status_code = 1;
                continue;
            }
        };

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
