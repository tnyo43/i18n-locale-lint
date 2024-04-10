use serde_json::{self};
use std::collections::HashMap;

use i18n_locale_lint_ast::value::{Value, Literal};

pub fn parse(content: &str) -> i18n_locale_lint_ast::value::Value {
    let serde_value = serde_json::from_str(content).unwrap();

    convert(&serde_value)
}

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

#[test]
fn string_array() {
    use insta::assert_snapshot;

    let content = r#"
{ 
  "dayNames": ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]
}
    "#;

    assert_snapshot!(parse(content),
    @r#"{
  "dayNames": [
    "Sun",
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat",
  ],
}"#
    );
}

#[test]
fn object_with_types() {
    use insta::assert_snapshot;

    let content = r#"
{
  "currency": {
    "format": "%{unit}%{number}",
    "delimiter": ",",
    "precision": 3,
    "strip_insignificant_zeros": false,
    "unit": "$"
  }
}
    "#;

    assert_snapshot!(parse(content),
    @r#"{
  "currency": {
    "delimiter": ",",
    "format": "%{unit}%{number}",
    "precision": 3,
    "strip_insignificant_zeros": false,
    "unit": "$",
  },
}"#
    );
}

#[test]
fn nested_object() {
    use insta::assert_snapshot;

    let content = r#"
{
  "date": {
    "dayNames": ["Sun", "Mon", "..."],
    "monthNames": [{ "full": "January", "short": "Jan" }, { "full": "February", "short": "Feb" }],
    "formats": {
      "default": "yyyy/MM/dd",
      "monthDay": "MM/dd",
      "yearMonth": "yyyy/MM",
      "time": "hh:mm",
      "full": "yyyy/MM/dd hh:mm"
    }
  },
  "error": {
    "format": "%{attribute} ${message}",
    "messages": {
      "too long": {
        "one": "is too long (maximum is %{count} character)",
        "other": "is too long (maximum is %{count} characters)"
      }
    }
  }
}
    "#;

    assert_snapshot!(
        parse(content),
    @r#"{
  "date": {
    "dayNames": [
      "Sun",
      "Mon",
      "...",
    ],
    "formats": {
      "default": "yyyy/MM/dd",
      "full": "yyyy/MM/dd hh:mm",
      "monthDay": "MM/dd",
      "time": "hh:mm",
      "yearMonth": "yyyy/MM",
    },
    "monthNames": [
      {
        "full": "January",
        "short": "Jan",
      },
      {
        "full": "February",
        "short": "Feb",
      },
    ],
  },
  "error": {
    "format": "%{attribute} ${message}",
    "messages": {
      "too long": {
        "one": "is too long (maximum is %{count} character)",
        "other": "is too long (maximum is %{count} characters)",
      },
    },
  },
}"#
    );
}
