use serde_json::Value;
use termion::color;

use crate::diff::Diff;

fn display_value(value: &Value, indent: &str) {
    match value {
        Value::Number(n) => {
            print!("{}(number)", n);
        }
        Value::Null => {
            print!("null");
        }
        Value::Bool(b) => {
            print!("{}(bool)", b);
        }
        Value::String(s) => {
            print!("\"{}\"(string)", s);
        }
        Value::Array(_) => todo!(),
        Value::Object(o) => {
            let new_indent = indent.to_string() + "  ";
            println!("{{");
            for key in o.keys() {
                print!("{}{}: ", new_indent, key);
                display_value(&o[key], &new_indent);
                println!(",");
            }
            print!("{}}}", indent);
        }
    }
}

fn display_some_value(value: Option<&Value>) {
    if let Some(v) = value {
        print!("{}", color::Fg(color::Green));
        display_value(v, "");
        println!("{}", color::Fg(color::Reset));
    } else {
        println!(
            "{}not set{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Reset)
        );
    }
}

pub fn display_diff(diff: &Diff, file1: &str, file2: &str) {
    println!(
        "compare {}{}{}",
        color::Fg(color::Yellow),
        file1,
        color::Fg(color::Reset)
    );
    println!(
        "   with {}{}{}",
        color::Fg(color::Yellow),
        file2,
        color::Fg(color::Reset)
    );
    println!(
        "detect a type difference with this key: {}{}{}",
        color::Fg(color::Yellow),
        diff.key_to_value.join("."),
        color::Fg(color::Reset)
    );
    println!("\nin {}", file1);
    display_some_value(diff.expected);
    println!("\nin {}", file2);
    display_some_value(diff.actual);
    println!();
}
