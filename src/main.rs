mod check;
mod files;
mod option;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = args[1].as_str();

    let option = option::Option::from_cli(&args);
    let _ = option::INSTANCE.set(option);

    let mut ok = true;
    match files::get_file_groups(pattern) {
        Ok(file_groups) => {
            for group in file_groups {
                ok &= check::check(&group.1.iter().map(|p| p.as_str()).collect());
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
