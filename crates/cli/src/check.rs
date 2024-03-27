use json_inspector_ast::value::Value;

use crate::option;

fn read_json_file(path: &str) -> Value {
    json_inspector_json::parse::get_json_data(path)
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
        let result = base_data.compare(&mut vec![], &target_data);
        if let Some(diff) = result {
            diff.display(base_file_path, target_file_path);
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
