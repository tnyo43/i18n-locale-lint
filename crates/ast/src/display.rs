use crate::value::{Diff, Literal, Value};
use colored::*;

impl Literal {
    fn display(&self) {
        match self {
            Literal::Number(n) => {
                print!("{}", n.to_string().blue());
            }
            Literal::Null => {
                print!("{}", "null".blue());
            }
            Literal::Bool(b) => {
                print!("{}", b.to_string().blue());
            }
            Literal::String(s) => {
                print!("{}{}{}", "\"".blue(), s.blue(), "\"".blue());
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
                println!("{}", "[".blue());
                for v in arr {
                    print!("{}", indent);
                    v.display(&new_indent);
                    println!("{}", ",".blue());
                }
                println!("{}]", indent);
            }
            Value::Map(map) => {
                let new_indent = indent.to_string() + "  ";
                println!("{}", "{".blue());
                for key in map.keys() {
                    print!("{}{}{} ", new_indent.blue(), key.blue(), ":".blue());
                    map[key].display(&new_indent);
                    println!("{}", ",".blue());
                }
                print!("{}{}", indent, "}".blue());
            }
        }
    }
}

fn display_some_value(value: &Option<Value>) {
    if let Some(v) = value {
        v.display("");
        println!();
    } else {
        println!("{}", "not set".blue());
    }
}

impl Diff {
    pub fn display(&self, file1: &str, file2: &str) {
        println!(
            "{} {}",
            "detected a type difference with this key:".red(),
            self.key_to_value.join(".").red(),
        );
        println!("\nin {}", file1);
        display_some_value(&self.expected);
        println!("\nin {}", file2);
        display_some_value(&self.actual);
        println!();
    }
}
