use std::cmp::Ordering;

use crate::value::{Diff, Value};

impl Value {
    pub fn compare(&self, key_to_value: &mut Vec<String>, other: &Self) -> Option<Diff> {
        if let Self::Literal(_) = self {
            if let Self::Literal(_) = other {
                return None;
            }
        } else if let Self::Array(arr1) = self {
            if let Self::Array(arr2) = other {
                match arr1.len().cmp(&arr2.len()) {
                    Ordering::Greater => {
                        key_to_value.push(arr2.len().to_string());
                        return Some(Diff {
                            key_to_value: key_to_value.to_vec(),
                            expected: Some(arr1[arr2.len()].clone()),
                            actual: None,
                        });
                    }
                    Ordering::Less => {
                        key_to_value.push(arr1.len().to_string());
                        return Some(Diff {
                            key_to_value: key_to_value.to_vec(),
                            expected: None,
                            actual: Some(arr2[arr1.len()].clone()),
                        });
                    }
                    Ordering::Equal => {
                        for i in 0..arr1.len() {
                            key_to_value.push(i.to_string());
                            let result = arr1[i].compare(key_to_value, &arr2[i]);
                            if result.is_some() {
                                return result;
                            }
                            key_to_value.pop();
                        }
                        return None;
                    }
                }
            }
        } else if let Self::Map(map1) = self {
            if let Self::Map(map2) = other {
                for key in map1.keys() {
                    key_to_value.push(key.to_string());

                    if !map2.contains_key(key) {
                        let expected = Some(*map1[key].clone());
                        return Some(Diff {
                            key_to_value: key_to_value.to_vec(),
                            expected,
                            actual: None,
                        });
                    }

                    let result = map1[key].compare(key_to_value, &map2[key]);
                    if result.is_some() {
                        return result;
                    }
                    key_to_value.pop();
                }

                for key in map2.keys() {
                    if !map1.contains_key(key) {
                        key_to_value.push(key.to_string());
                        return Some(Diff {
                            key_to_value: key_to_value.clone(),
                            expected: None,
                            actual: Some(*map2[key].clone()),
                        });
                    }
                }
                return None;
            }
        }

        Some(Diff {
            key_to_value: key_to_value.to_vec(),
            expected: Some(self.clone()),
            actual: Some(other.clone()),
        })
    }
}
