use serde_json::Value;
use crate::is_same_type::Diff;

fn display_some_value(value: Option<&Value>) {
    if let Some(v) = value {
        println!("{}", v);
    } else {
        println!("nothing");
    }
}

pub fn display_diff(diff: &Diff, file1: &str, file2: &str) {
    println!("key: {}", diff.key_to_value.join("."));
    println!("\nin {}", file1);
    display_some_value(diff.expected);
    println!("\nin {}", file2);
    display_some_value(diff.actual);
    println!("\n");
}
