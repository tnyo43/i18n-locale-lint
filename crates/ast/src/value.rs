use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Array(Vec<Value>),
    Map(HashMap<String, Box<Value>>),
}

impl Value {
    pub fn skip_top_level(&self) -> Self {
        if let Self::Map(map) = self {
            if map.keys().len() != 1 {
                panic!();
            }
            let key = map.keys().collect::<Vec<_>>()[0];

            return *map.get(key).unwrap().clone();
        } else {
            panic!()
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    pub key_to_value: Vec<String>,
    pub expected: Option<Value>,
    pub actual: Option<Value>,
}
