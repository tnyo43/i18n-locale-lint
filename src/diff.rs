use serde_json::Value;

#[derive(Debug, PartialEq)]
pub struct Diff<'a> {
    pub key_to_value: Vec<String>,
    pub expected: Option<&'a Value>,
    pub actual: Option<&'a Value>,
}
