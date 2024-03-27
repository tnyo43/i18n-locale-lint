use termion::color;

use crate::value::{Diff, Literal, Value};

impl Literal {
    fn display(&self) {
        match self {
            Literal::Number(n) => {
                print!("{}(number)", n);
            }
            Literal::Null => {
                print!("null");
            }
            Literal::Bool(b) => {
                print!("{}(bool)", b);
            }
            Literal::String(s) => {
                print!("\"{}\"(string)", s);
            }
        }
    }
}

impl Value {
    fn display(&self, indent: &str) {
        match self {
            Value::Literal(l) => l.display(),
            Value::Array(arr) => {
                let new_indent = indent.to_string() + "  ";
                println!("[");
                for v in arr {
                    print!("{}", indent);
                    v.display(&new_indent);
                    println!(",");
                }
                println!("{}]", indent);
            }
            Value::Map(map) => {
                let new_indent = indent.to_string() + "  ";
                println!("{{");
                for key in map.keys() {
                    print!("{}{}: ", new_indent, key);
                    map[key].display(&new_indent);
                    println!(",");
                }
                print!("{}}}", indent);
            }
        }
    }
}

fn display_some_value(value: &Option<Value>) {
    if let Some(v) = value {
        print!("{}", color::Fg(color::Green));
        v.display("");
        println!("{}", color::Fg(color::Reset));
    } else {
        println!(
            "{}not set{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Reset)
        );
    }
}

impl Diff {
    pub fn display(&self, file1: &str, file2: &str) {
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
            self.key_to_value.join("."),
            color::Fg(color::Reset)
        );
        println!("\nin {}", file1);
        display_some_value(&self.expected);
        println!("\nin {}", file2);
        display_some_value(&self.actual);
        println!();
    }
}
