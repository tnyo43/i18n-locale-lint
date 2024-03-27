use std::collections::HashMap;

use json_inspector_ast::value::{Value, Literal};

pub fn convert(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::String(s) => Value::Literal(Literal::String(s.clone())),
        serde_json::Value::Bool(b) => Value::Literal(Literal::Bool(*b)),
        serde_json::Value::Number(n) => Value::Literal(Literal::Number(n.as_f64().unwrap())),
        serde_json::Value::Null => Value::Literal(Literal::Null),
        serde_json::Value::Array(arr) => Value::Array(arr.iter().map(convert).collect()),
        serde_json::Value::Object(obj) => Value::Map(HashMap::from_iter(
            obj.iter()
                .map(|(key, value)| (key.clone(), Box::new(convert(value)))),
        )),
    }
}
