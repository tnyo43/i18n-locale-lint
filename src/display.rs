use serde_json::Value;
use termion::color;
use crate::is_same_type::Diff;

fn display_some_value(value: Option<&Value>) {
    if let Some(v) = value {
        println!("{}{}{}", color::Fg(color::Green), v, color::Fg(color::Reset));
    } else {
        println!("{}nothing{}", color::Fg(color::LightBlack), color::Fg(color::Reset));
    }
}

pub fn display_diff(diff: &Diff, file1: &str, file2: &str) {
    println!("compare {}{}{}", color::Fg(color::Yellow), file1, color::Fg(color::Reset));
    println!("   with {}{}{}", color::Fg(color::Yellow), file2, color::Fg(color::Reset));
    println!("key: {}", diff.key_to_value.join("."));
    println!("\nin {}", file1);
    display_some_value(diff.expected);
    println!("\nin {}", file2);
    display_some_value(diff.actual);
    println!("\n");
}
