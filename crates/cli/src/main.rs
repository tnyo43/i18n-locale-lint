use std::{env, process::exit};

mod check;
mod error;
mod files;
pub mod option;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let option = option::CliOption::from_cli(&args);
    let _ = option::INSTANCE.set(option);

    let mut failure_count = 0;

    let file_groups = files::get_file_groups();
    for group in &file_groups {
        let status_code = check::check(&group.1.iter().map(|p| p.as_str()).collect());
        if status_code != 0 {
            failure_count += 1
        }
    }
    let status_code = if failure_count == 0 { 0 } else { 1 };

    println!(
        "\nChecked {} files, {} groups",
        option::INSTANCE.get().unwrap().files.len(),
        &file_groups.len()
    );
    if status_code != 0 {
        print!("\x1b[31m");
    } else {
        print!("\x1b[32m");
    }
    println!(
        "Found {} mismatched group{}",
        failure_count,
        if failure_count >= 2 { "s" } else { "" }
    );
    print!("\x1b[m");

    exit(status_code)
}
