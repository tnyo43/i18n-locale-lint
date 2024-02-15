pub mod is_same_type;

use std::env;
use std::fmt::Display;
use std::fs;
use is_same_type::Diff;
use serde_json::Value;

fn read_json_file(path: &str) -> Value {
    let data = fs::read_to_string(path);
    let content = if let Ok(content) = data { content } else { "failed to parse".to_string() };

    let parsed_data: Value = serde_json::from_str(&content).unwrap();

    parsed_data
}


impl Display for Diff<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Diff: key_to_value={:?}, expected={:?}, actual={:?}", self.key_to_value, self.expected, self.actual)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: my_cli_tool <arg1> <arg2>");
        return;
    }

    let file1 = &args[1];
    let file2 = &args[2];

    let data1 = read_json_file(file1);
    let data2 = read_json_file(file2);

    let result = is_same_type::is_same_type(&data1, &data2, &mut vec![]);
    if let Some(diff) = result {
        println!("{}", diff);
    }
}
