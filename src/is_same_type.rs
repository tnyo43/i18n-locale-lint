use serde_json::Value;

use crate::diff::Diff;

pub fn is_same_type<'a>(
    base: &'a Value,
    target: &'a Value,
    key_to_value: &mut Vec<String>,
) -> Option<Diff<'a>> {
    if let Some(base_object) = base.as_object() {
        if let Some(target_object) = target.as_object() {
            for key in base_object.keys() {
                key_to_value.push(key.to_string());

                if !target_object.contains_key(key) {
                    return Some(Diff {
                        key_to_value: key_to_value.to_vec(),
                        expected: Some(&base_object[key]),
                        actual: None,
                    });
                }

                let result = is_same_type(&base_object[key], &target_object[key], key_to_value);
                if result.is_some() {
                    return result;
                }
                key_to_value.pop();
            }

            for key in target_object.keys() {
                if !base_object.contains_key(key) {
                    key_to_value.push(key.to_string());
                    return Some(Diff {
                        key_to_value: key_to_value.to_vec(),
                        expected: None,
                        actual: Some(&target_object[key]),
                    });
                }
            }
            return None;
        }
    }

    if base.is_boolean() && target.is_boolean()
        || base.is_number() && target.is_number()
        || base.is_null() && target.is_null()
        || base.is_string() && target.is_string()
    {
        return None;
    }

    Some(Diff {
        key_to_value: key_to_value.to_vec(),
        expected: Some(base),
        actual: Some(target),
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};
    use crate::is_same_type::{is_same_type, Diff};

    fn value_string() -> Value {
        "lorem".to_string().into()
    }

    fn value_number() -> Value {
        1.into()
    }

    fn value_boolean() -> Value {
        false.into()
    }

    fn value_null() -> Value {
        ().into()
    }

    fn value_object() -> Value {
        let mut m = Map::new();
        m.insert("lorem".to_string(), "ipsum".into());
        m.into()
    }

    #[test]
    fn none_if_same_primitive() {
        let mut key = vec!["key".to_string(), "to".to_string()];
        assert_eq!(
            is_same_type(&value_string(), &"ipsum".into(), &mut key),
            None
        );
        assert_eq!(is_same_type(&value_number(), &3.into(), &mut key), None);
        assert_eq!(is_same_type(&value_boolean(), &true.into(), &mut key), None);
        assert_eq!(is_same_type(&value_null(), &().into(), &mut key), None);
    }

    #[test]
    fn diff_if_different_primitive() {
        let mut key = vec!["key".to_string(), "to".to_string()];
        assert_eq!(
            is_same_type(&value_string(), &3.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_string()),
                actual: Some(&3.into())
            })
        );
        assert_eq!(
            is_same_type(&value_string(), &true.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_string()),
                actual: Some(&true.into())
            })
        );
        assert_eq!(
            is_same_type(&value_string(), &().into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_string()),
                actual: Some(&().into())
            })
        );

        assert_eq!(
            is_same_type(&value_number(), &"ipsum".into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_number()),
                actual: Some(&"ipsum".into())
            })
        );
        assert_eq!(
            is_same_type(&value_number(), &true.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_number()),
                actual: Some(&true.into())
            })
        );
        assert_eq!(
            is_same_type(&value_number(), &().into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_number()),
                actual: Some(&().into())
            })
        );

        assert_eq!(
            is_same_type(&value_boolean(), &"ipsum".into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_boolean()),
                actual: Some(&"ipsum".into())
            })
        );
        assert_eq!(
            is_same_type(&value_boolean(), &3.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_boolean()),
                actual: Some(&3.into())
            })
        );
        assert_eq!(
            is_same_type(&value_boolean(), &().into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_boolean()),
                actual: Some(&().into())
            })
        );

        assert_eq!(
            is_same_type(&value_null(), &"ipsum".into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_null()),
                actual: Some(&"ipsum".into())
            })
        );
        assert_eq!(
            is_same_type(&value_null(), &3.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_null()),
                actual: Some(&3.into())
            })
        );
        assert_eq!(
            is_same_type(&value_null(), &true.into(), &mut key),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string()],
                expected: Some(&value_null()),
                actual: Some(&true.into())
            })
        );
    }

    #[test]
    fn none_if_compare_primitive_with_object() {
        let mut key: Vec<String> = vec!["key".to_string(), "to".to_string()];
        for value in [
            value_string(),
            value_number(),
            value_boolean(),
            value_null(),
        ] {
            assert_eq!(
                is_same_type(&value, &value_object(), &mut key),
                Some(Diff {
                    key_to_value: vec!["key".to_string(), "to".to_string()],
                    expected: Some(&value),
                    actual: Some(&value_object())
                })
            );
            assert_eq!(
                is_same_type(&value_object(), &value, &mut key),
                Some(Diff {
                    key_to_value: vec!["key".to_string(), "to".to_string()],
                    expected: Some(&value_object()),
                    actual: Some(&value)
                })
            );
        }
    }

    #[test]
    fn diff_if_same_object() {
        let mut key: Vec<String> = vec!["key".to_string(), "to".to_string()];

        // m1 = { "lorem": "ipsum" }
        // m2 = { "lorem": "apple" }
        let mut m1 = Map::new();
        m1.insert("lorem".to_string(), "ipsum".into());
        let value_map1: Value = m1.into();
        let mut m2 = Map::new();
        m2.insert("lorem".to_string(), "apple".into());
        let value_map2: Value = m2.into();

        assert_eq!(is_same_type(&value_map1, &value_map2, &mut key), None);

        // m3 = { "name": "Alice", "age": "10", "favorite": { "food": "apple", "number": 7 }, "occupation": null }
        // m4 = { "name": "Bob", "age": "12", "favorite": { "food": "banana", "number": 100 }, "occupation": null }
        let mut m3 = Map::new();
        m3.insert("name".to_string(), "Alice".into());
        m3.insert("age".to_string(), 10.into());
        let mut favorite3 = Map::new();
        favorite3.insert("food".to_string(), "apple".into());
        favorite3.insert("number".to_string(), 7.into());
        m3.insert("favorite".to_string(), favorite3.into());
        m3.insert("occupation".to_string(), ().into());
        let value_map3: Value = m3.into();

        let mut m4: Map<String, Value> = Map::new();
        m4.insert("name".to_string(), "Bob".into());
        m4.insert("age".to_string(), 12.into());
        let mut favorite4 = Map::new();
        favorite4.insert("food".to_string(), "banana".into());
        favorite4.insert("number".to_string(), 100.into());
        m4.insert("favorite".to_string(), favorite4.into());
        m4.insert("occupation".to_string(), ().into());
        let value_map4: Value = m4.into();

        assert_eq!(is_same_type(&value_map3, &value_map4, &mut key), None);
    }

    #[test]
    fn diff_if_different_object() {
        let key: Vec<String> = vec!["key".to_string(), "to".to_string()];
        // m1 = { "lorem": "ipsum" }
        // m1 = { "foo": "bar" }
        let mut m1 = Map::new();
        m1.insert("lorem".to_string(), "ipsum".into());
        let value_map1: Value = m1.into();
        let mut m2: Map<String, Value> = Map::new();
        m2.insert("foo".to_string(), "bar".into());
        let value_map2: Value = m2.into();

        assert_eq!(
            is_same_type(&value_map1, &value_map2, &mut key.clone()),
            Some(Diff {
                key_to_value: vec!["key".to_string(), "to".to_string(), "lorem".to_string()],
                expected: Some(&"ipsum".into()),
                actual: None
            })
        );

        // m3 = { "name": "Alice", "age": "10", "favorite": { "food": "apple" }, "occupation": null }
        // m4 = { "name": "Bob", "age": "12", "favorite": { "food": "banana", "number": 100 }, "occupation": null }
        let mut m3 = Map::new();
        m3.insert("name".to_string(), "Alice".into());
        m3.insert("age".to_string(), 10.into());
        let mut favorite3 = Map::new();
        favorite3.insert("food".to_string(), "apple".into());
        m3.insert("favorite".to_string(), favorite3.into());
        m3.insert("occupation".to_string(), ().into());
        let value_map3: Value = m3.into();

        let mut m4: Map<String, Value> = Map::new();
        m4.insert("name".to_string(), "Bob".into());
        m4.insert("age".to_string(), 12.into());
        let mut favorite4 = Map::new();
        favorite4.insert("food".to_string(), "banana".into());
        favorite4.insert("number".to_string(), 100.into());
        m4.insert("favorite".to_string(), favorite4.into());
        m4.insert("occupation".to_string(), ().into());
        let value_map4: Value = m4.into();

        assert_eq!(
            is_same_type(&value_map3, &value_map4, &mut key.clone()),
            Some(Diff {
                key_to_value: vec![
                    "key".to_string(),
                    "to".to_string(),
                    "favorite".to_string(),
                    "number".to_string()
                ],
                expected: None,
                actual: Some(&100.into())
            })
        );
    }
}
