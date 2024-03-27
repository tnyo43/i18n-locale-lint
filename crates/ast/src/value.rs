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

#[derive(Debug)]
pub struct Diff {
    pub key_to_value: Vec<String>,
    pub expected: Option<Value>,
    pub actual: Option<Value>,
}
