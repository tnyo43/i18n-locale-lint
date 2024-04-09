use std::collections::HashMap;

use i18n_locale_lint_ast::value::{Value, Literal};

pub fn parse(content: String) -> i18n_locale_lint_ast::value::Value {
    let serde_value = serde_yaml::from_str(&content).unwrap();

    convert(&serde_value)
}

fn convert(value: &serde_yaml::Value) -> Value {
    match value {
        serde_yaml::Value::String(s) => Value::Literal(Literal::String(s.clone())),
        serde_yaml::Value::Bool(b) => Value::Literal(Literal::Bool(*b)),
        serde_yaml::Value::Number(n) => Value::Literal(Literal::Number(n.as_f64().unwrap())),
        serde_yaml::Value::Null => Value::Literal(Literal::Null),
        serde_yaml::Value::Sequence(arr) => Value::Array(arr.iter().map(convert).collect()),
        serde_yaml::Value::Mapping(obj) => {
            Value::Map(HashMap::from_iter(obj.iter().map(|(key, value)| {
                (key.as_str().unwrap().to_string(), Box::new(convert(value)))
            })))
        }
        serde_yaml::Value::Tagged(_) => todo!("yaml tag is not able to parse yet."),
    }
}

#[test]
fn string_array() {
    use insta::assert_snapshot;

    let content = "\
dayNames:
- Sun
- Mon
- Tue
- Wed
- Thu
- Fri
- Sat
    "
    .to_string();

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
currency:
  format: "%{unit}%{number}"
  delimiter: ","
  precision: 3
  strip_insignificant_zeros: false
  unit: $
"#
    .to_string();

    println!("{}", parse(content.clone()));

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
date:
  dayNames:
    - Sun
    - Mon
    - "..."
  monthNames:
    - full: January
      short: Jan
    - full: February
      short: Feb
  formats:
    default: yyyy/MM/dd
    monthDay: MM/dd
    yearMonth: yyyy/MM
    time: hh:mm
    full: yyyy/MM/dd hh:mm
error:
  format: "%{attribute} ${message}"
  messages:
    too long:
      one: is too long (maximum is %{count} character)
      other: is too long (maximum is %{count} characters)
    "#
    .to_string();

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
