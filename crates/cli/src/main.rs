use std::{env, process::exit};

mod check;
mod error;
mod files;
pub mod option;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let option = option::CliOption::from_cli(&args);
    let _ = option::INSTANCE.set(option);

    let mut status_code = 0;

    let file_groups = files::get_file_groups();
    for group in file_groups {
        status_code |= check::check(&group.1.iter().map(|p| p.as_str()).collect());
    }

    exit(status_code)
}
