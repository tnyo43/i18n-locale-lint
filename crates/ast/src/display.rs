use std::fmt::Display;

use crate::value::{Diff, Literal, Value};

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Null => write!(f, "null"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl Value {
    fn fmt_sub(&self, f: &mut std::fmt::Formatter<'_>, indent: &str) -> std::fmt::Result {
        match self {
            Value::Literal(l) => l.fmt(f),
            Value::Array(arr) => {
                let new_indent = indent.to_string() + "  ";
                let _ = writeln!(f, "[");
                for v in arr {
                    let _ = write!(f, "{}", indent);
                    let _ = v.fmt_sub(f, &new_indent);
                    let _ = writeln!(f, ",");
                }
                write!(f, "{}]", indent)
            }
            Value::Map(map) => {
                let new_indent = indent.to_string() + "  ";
                let _ = writeln!(f, "{{");
                for key in map.keys() {
                    let _ = write!(f, "{}{}: ", new_indent, key);
                    let _ = map[key].fmt_sub(f, &new_indent);
                    let _ = writeln!(f, ",");
                }
                write!(f, "{}}}", indent,)
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_sub(f, "")
    }
}

fn display_some_value(value: &Option<Value>) {
    // print with blue text
    print!("\x1b[34m");
    if let Some(v) = value {
        println!("{}", v);
    } else {
        println!("not set");
    }
    print!("\x1b[m");
}

impl Diff {
    pub fn display(&self, file1: &str, file2: &str) {
        // print with red text
        println!(
            "\x1b[31mdetected a type difference with this key: {}\x1b[m",
            self.key_to_value.join("."),
        );
        println!("\nin {}", file1);
        display_some_value(&self.expected);
        println!("\nin {}", file2);
        display_some_value(&self.actual);
        println!();
    }
}
