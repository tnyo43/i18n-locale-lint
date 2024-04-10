use std::collections::HashMap;

use i18n_locale_lint_ast::value::{Value, Literal};

pub fn parse(content: String) -> i18n_locale_lint_ast::value::Value {
    let serde_value = serde_yaml::from_str(&content).unwrap();

    convert(&serde_value)
}

pub fn to_string(key: &serde_yaml::Value) -> String {
    match key {
        serde_yaml::Value::Null => "null".to_string(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::String(s) => s.to_string(),
        serde_yaml::Value::Tagged(t) => {
            format!("{} {}", &t.tag.to_string(), to_string(&t.value))
        }
        serde_yaml::Value::Sequence(arr) => format!(
            "[{}]",
            arr.iter().map(to_string).collect::<Vec<String>>().join(",")
        ),
        serde_yaml::Value::Mapping(obj) => format!(
            "{{{}}}",
            obj.iter()
                .map(|(key, value)| format!("{}:{}", to_string(key), to_string(value)))
                .collect::<Vec<String>>()
                .join(",")
        ),
    }
}

fn convert(value: &serde_yaml::Value) -> Value {
    match value {
        serde_yaml::Value::String(s) => Value::Literal(Literal::String(s.clone())),
        serde_yaml::Value::Bool(b) => Value::Literal(Literal::Bool(*b)),
        serde_yaml::Value::Number(n) => Value::Literal(Literal::Number(n.as_f64().unwrap())),
        serde_yaml::Value::Null => Value::Literal(Literal::Null),
        serde_yaml::Value::Tagged(t) => Value::Literal(Literal::String(format!(
            "{} {}",
            &t.tag.to_string(),
            to_string(&t.value)
        ))),
        serde_yaml::Value::Sequence(arr) => Value::Array(arr.iter().map(convert).collect()),
        serde_yaml::Value::Mapping(obj) => Value::Map(HashMap::from_iter(
            obj.iter()
                .map(|(key, value)| (to_string(key), Box::new(convert(value)))),
        )),
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
    default: !DateFormat yyyy/MM/dd
    monthDay: !DateFormat MM/dd
    yearMonth: !DateFormat yyyy/MM
    time: !TimeFormat hh:mm
    full: !DateTimeFormat yyyy/MM/dd hh:mm
answer:
  True: Yes
  False: No
  0: Zero
  1: One
  Null: Null
!Tag 0: tag 0
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
  "!Tag 0": "tag 0",
  "answer": {
    "0": "Zero",
    "1": "One",
    "false": "No",
    "null": null,
    "true": "Yes",
  },
  "date": {
    "dayNames": [
      "Sun",
      "Mon",
      "...",
    ],
    "formats": {
      "default": "!DateFormat yyyy/MM/dd",
      "full": "!DateTimeFormat yyyy/MM/dd hh:mm",
      "monthDay": "!DateFormat MM/dd",
      "time": "!TimeFormat hh:mm",
      "yearMonth": "!DateFormat yyyy/MM",
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
