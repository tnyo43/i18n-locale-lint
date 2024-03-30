use std::{env, process::exit};

mod check;
mod files;
pub mod option;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let option = option::Option::from_cli(&args);
    let _ = option::INSTANCE.set(option);

    let mut status_code = 0;
    match files::get_file_groups() {
        Ok(file_groups) => {
            println!("-> {:?}", file_groups.keys());
            for group in file_groups {
                status_code |= check::check(&group.1.iter().map(|p| p.as_str()).collect());
            }
        }
        Err(e) => {
            println!("{:?}", e);
            status_code = 1;
        }
    }

    exit(status_code)
}
