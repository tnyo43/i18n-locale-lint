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

    pub fn skip_top_level(&self) -> Self {
        if let Self::Map(map) = self {
            if map.is_empty() {
                panic!("expect the top level value is a singleton map, but is empty");
            }
            if map.keys().len() != 1 {
                let keys = map
                    .keys()
                    .map(|s| "\"".to_string() + s + "\"")
                    .sorted()
                    .join(", ");
                panic!(
                    "expect the top level value is a singleton map, but it has multiple keys: {}",
                    keys
                );
            }
            let key = map.keys().collect::<Vec<_>>()[0];

            return *map.get(key).unwrap().clone();
        } else {
            panic!(
                "expect the top level value is a singleton map, but is actually a {}",
                self.value_type()
            );
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
        #[should_panic(
            expected = "expect the top level value is a singleton map, but is actually a literal"
        )]
        fn if_the_value_is_literal() {
            let value = Value::Literal(Literal::Null);
            value.skip_top_level();
        }

        #[test]
        #[should_panic(
            expected = "expect the top level value is a singleton map, but is actually a array"
        )]
        fn if_the_value_is_array() {
            let value = Value::Array(vec![]);
            value.skip_top_level();
        }
    }

    mod should_panic_if_value_is_not_a_singleton_map {
        pub use std::collections::HashMap;
        pub use crate::value::{Literal, Value};

        #[test]
        #[should_panic(expected = "expect the top level value is a singleton map, but is empty")]
        fn if_the_value_is_an_empty_map() {
            let value = Value::Map(HashMap::from([]));
            value.skip_top_level();
        }

        #[test]
        #[should_panic(
            expected = "expect the top level value is a singleton map, but it has multiple keys: \"greeting\", \"name\", \"phone\""
        )]
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
            value.skip_top_level();
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
            Value::Map(HashMap::from([
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
            ]))
        );
    }
}
