use std::env;
use std::fs;
use serde_json::{Result, Value};


fn read_json_file(path: &str) -> Result<Value> {
    let data = fs::read_to_string(path);
    let content = if let Ok(content) = data { content } else { "failed to parse".to_string() };

    let parsed_data: Value = serde_json::from_str(&content)?;

    println!("{:?}", parsed_data);
    Ok(parsed_data)
}

fn main() {
    let _ = read_json_file("example/data1.json");
    let _ = read_json_file("example/data2.json");

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
