use std::env;
use std::fs;
use serde_json::{Result, Value};


fn read_json() -> Result<()> {
    let data = fs::read_to_string("data.json");
    if let Ok(x) = data {
        let parsed_data: Value = serde_json::from_str(&x)?;

        println!("{:?}", parsed_data);
    }

    Ok(())
}

fn main() {
    let _ = read_json();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: my_cli_tool <arg1> <arg2>");
        return;
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    println!("Argument 1: {}", arg1);
    println!("Argument 2: {}", arg2);
}
