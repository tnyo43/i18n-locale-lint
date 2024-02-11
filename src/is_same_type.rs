use serde_json::Value;

pub fn is_same_type(base: &Value, target: &Value) -> bool {
    if let Some(base_object) = base.as_object() {
        if let Some(target_object) = target.as_object() {
            for key in base_object.keys() {
                if !target_object.contains_key(key) {
                    return false
                }
                if !is_same_type(&base_object[key], &target_object[key]) {
                    return false
                }
            }

            for key in target_object.keys() {
                if !base_object.contains_key(key) {
                    return false
                }
            }
            return true
        } else {
            return false
        }
    }

    if base.is_boolean() && target.is_boolean() || base.is_number() && target.is_number() || base.is_null() && target.is_null() || base.is_string() && target.is_string() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};
    use crate::is_same_type::is_same_type;

    fn value_string () -> Value {
        "lorem".to_string().into()
    }

    fn value_number () -> Value {
        1.into()
    }

    fn value_boolean () -> Value {
        false.into()
    }

    fn value_null () -> Value {
        ().into()
    }

    fn value_object () -> Value {
        let mut m = Map::new();
        m.insert("lorem".to_string(), "ipsum".into());
        m.into()
    }

    #[test]
    fn true_if_same_primitive() {
        assert!(is_same_type(&value_string(), &"ipsum".into()));
        assert!(is_same_type(&value_number(), &3.into()));
        assert!(is_same_type(&value_boolean(), &true.into()));
        assert!(is_same_type(&value_null(), &().into()));
    }

    #[test]
    fn false_if_different_primitive() {
        assert!(!is_same_type(&value_string(), &3.into()));
        assert!(!is_same_type(&value_string(), &true.into()));
        assert!(!is_same_type(&value_string(), &().into()));

        assert!(!is_same_type(&value_number(), &"ipsum".into()));
        assert!(!is_same_type(&value_number(), &true.into()));
        assert!(!is_same_type(&value_number(), &().into()));

        assert!(!is_same_type(&value_boolean(), &"ipsum".into()));
        assert!(!is_same_type(&value_boolean(), &3.into()));
        assert!(!is_same_type(&value_boolean(), &().into()));

        assert!(!is_same_type(&value_null(), &"ipsum".into()));
        assert!(!is_same_type(&value_null(), &3.into()));
        assert!(!is_same_type(&value_null(), &true.into()));
    }

    #[test]
    fn false_if_compare_primitive_with_object() {
        for v in vec![value_string(), value_number(), value_boolean(), value_null()] {
            assert!(!is_same_type(&v, &value_object()));
            assert!(!is_same_type(&value_object(), &v));
        }
    }

    #[test]
    fn true_if_same_object() {
        // m1 = { "lorem": "ipsum" }
        // m2 = { "lorem": "apple" }
        let mut m1 = Map::new();
        m1.insert("lorem".to_string(), "ipsum".into());
        let value_map1: Value = m1.into();
        let mut m2 = Map::new();
        m2.insert("lorem".to_string(), "apple".into());
        let value_map2: Value = m2.into();

        assert!(is_same_type(&value_map1, &value_map2));


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

        assert!(is_same_type(&value_map3, &value_map4));
    }

    #[test]
    fn false_if_different_object() {
        // m1 = { "lorem": "ipsum" }
        // m1 = { "foo": "bar" }
        let mut m1 = Map::new();
        m1.insert("lorem".to_string(), "ipsum".into());
        let value_map1: Value = m1.into();
        let mut m2: Map<String, Value> = Map::new();
        m2.insert("foo".to_string(), "bar".into());
        let value_map2: Value = m2.into();

        assert!(!is_same_type(&value_map1, &value_map2));


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

        assert!(!is_same_type(&value_map3, &value_map4));
    }
}
