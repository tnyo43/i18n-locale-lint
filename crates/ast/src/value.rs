use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Literal(Literal),
    Array(Vec<Value>),
    Map(HashMap<String, Box<Value>>),
}

impl Value {
    fn value_type(&self) -> &str {
        match self {
            Self::Literal(_) => "literal",
            Self::Array(_) => "array",
            Self::Map(_) => "map",
        }
    }

    pub fn skip_top_level(&self) -> Result<Self, String> {
        if let Self::Map(map) = self {
            if map.is_empty() {
                return Err(
                    "expect the top level value is a singleton map, but is empty".to_string(),
                );
            }
            if map.keys().len() != 1 {
                let keys = map
                    .keys()
                    .map(|s| "\"".to_string() + s + "\"")
                    .sorted()
                    .join(", ");
                return Err(format!(
                    "expect the top level value is a singleton map, but it has multiple keys: {}",
                    keys,
                ));
            }
            let key = map.keys().collect::<Vec<_>>()[0];

            return Ok(*map.get(key).unwrap().clone());
        } else {
            return Err(format!(
                "expect the top level value is a singleton map, but is actually a {}",
                self.value_type()
            ));
        }
    }
}

#[derive(Debug)]
pub struct Diff {
    pub key_to_value: Vec<String>,
    pub expected: Option<Value>,
    pub actual: Option<Value>,
}

#[cfg(test)]
mod skip_top_level_tests {
    use std::collections::HashMap;

    use super::{Literal, Value};

    mod should_panic_if_value_is_not_a_map {
        pub use crate::value::{Literal, Value};

        #[test]
        fn if_the_value_is_literal() {
            let value = Value::Literal(Literal::Null);
            assert_eq!(
                value.skip_top_level(),
                Err(
                    "expect the top level value is a singleton map, but is actually a literal"
                        .to_string()
                )
            );
        }

        #[test]
        fn if_the_value_is_array() {
            let value = Value::Array(vec![]);
            assert_eq!(
                value.skip_top_level(),
                Err(
                    "expect the top level value is a singleton map, but is actually a array"
                        .to_string()
                )
            );
        }
    }

    mod should_panic_if_value_is_not_a_singleton_map {
        pub use std::collections::HashMap;
        pub use crate::value::{Literal, Value};

        #[test]
        fn if_the_value_is_an_empty_map() {
            let value = Value::Map(HashMap::from([]));
            assert_eq!(
                value.skip_top_level(),
                Err("expect the top level value is a singleton map, but is empty".to_string())
            );
        }

        #[test]
        fn if_the_value_has_multiple_keys() {
            let value = Value::Map(HashMap::from([
                (
                    "name".to_string(),
                    Box::new(Value::Literal(Literal::String("Alice".to_string()))),
                ),
                (
                    "greeting".to_string(),
                    Box::new(Value::Literal(Literal::String("Hi".to_string()))),
                ),
                (
                    "phone".to_string(),
                    Box::new(Value::Literal(Literal::String("phone number".to_string()))),
                ),
            ]));
            assert_eq!(
                value.skip_top_level(),
                Err(
                  "expect the top level value is a singleton map, but it has multiple keys: \"greeting\", \"name\", \"phone\""
                        .to_string()
                )
            );
        }
    }

    #[test]
    fn skip_top_level() {
        let value = Value::Map(HashMap::from([(
            "name".to_string(),
            Box::new(Value::Map(HashMap::from([
                (
                    "name".to_string(),
                    Box::new(Value::Literal(Literal::String("Alice".to_string()))),
                ),
                (
                    "greeting".to_string(),
                    Box::new(Value::Literal(Literal::String("Hi".to_string()))),
                ),
                (
                    "phone".to_string(),
                    Box::new(Value::Literal(Literal::String("phone number".to_string()))),
                ),
            ]))),
        )]));
        let new_value = value.skip_top_level();
        assert_eq!(
            new_value,
            Ok(Value::Map(HashMap::from([
                (
                    "name".to_string(),
                    Box::new(Value::Literal(Literal::String("Alice".to_string()))),
                ),
                (
                    "greeting".to_string(),
                    Box::new(Value::Literal(Literal::String("Hi".to_string()))),
                ),
                (
                    "phone".to_string(),
                    Box::new(Value::Literal(Literal::String("phone number".to_string()))),
                ),
            ])))
        );
    }
}
