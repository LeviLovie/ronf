use crate::value::{Map, Table, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let mut yaml_content = yaml_rust2::YamlLoader::load_from_str(&content)
        .map_err(|e| format!("Failed to parse YAML: {}", e))?;
    let root = match yaml_content.len() {
        0 => yaml_rust2::Yaml::Hash(yaml_rust2::yaml::Hash::new()),
        1 => std::mem::replace(&mut yaml_content[0], yaml_rust2::Yaml::Null),
        n => {
            return Err(format!("Expected a single YAML document, but found {}", n));
        }
    };

    let mut map = Map::new();
    match root {
        yaml_rust2::Yaml::Hash(hash) => {
            for (key, value) in hash {
                if let yaml_rust2::Yaml::String(key_str) = key {
                    map.insert(key_str, from_yaml_value(&value));
                } else {
                    return Err("YAML keys must be strings".to_string());
                }
            }
        }
        _ => return Err("YAML root must be a mapping".to_string()),
    }
    Ok(map)
}

fn from_yaml_value(value: &yaml_rust2::Yaml) -> Value {
    match value {
        yaml_rust2::Yaml::Null => Value::None,
        yaml_rust2::Yaml::Boolean(b) => Value::Bool(*b),
        yaml_rust2::Yaml::Integer(i) => Value::Int(*i),
        yaml_rust2::Yaml::Real(n) => {
            if let Ok(i) = n.parse::<i64>() {
                Value::Int(i)
            } else {
                Value::Float(n.parse::<f64>().unwrap_or(0.0))
            }
        }
        yaml_rust2::Yaml::String(s) => Value::String(s.clone()),
        yaml_rust2::Yaml::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(from_yaml_value(item));
            }
            Value::Array(values)
        }
        yaml_rust2::Yaml::Hash(obj) => {
            let mut table = Table::new();
            for (key, value) in obj {
                table.insert(
                    key.clone().as_str().unwrap().to_string(),
                    from_yaml_value(value),
                );
            }
            Value::Table(table)
        }
        _ => Value::None,
    }
}

pub(crate) fn serialize(value: Map<String, Value>) -> String {
    let yaml_value = to_yaml_value(value);
    let mut out_str = String::new();
    let mut emitter = yaml_rust2::YamlEmitter::new(&mut out_str);
    emitter.dump(&yaml_value).unwrap();
    out_str
}

fn to_yaml_value(value: Map<String, Value>) -> yaml_rust2::Yaml {
    yaml_rust2::Yaml::Hash(
        value
            .into_iter()
            .map(|(k, v)| (yaml_rust2::Yaml::String(k), to_yaml_value_single(v)))
            .collect(),
    )
}

fn to_yaml_value_single(value: Value) -> yaml_rust2::Yaml {
    match value {
        Value::None => yaml_rust2::Yaml::Null,
        Value::Bool(b) => yaml_rust2::Yaml::Boolean(b),
        Value::Int(i) => yaml_rust2::Yaml::Integer(i),
        Value::Float(f) => yaml_rust2::Yaml::Real(f.to_string()),
        Value::String(s) => yaml_rust2::Yaml::String(s),
        Value::Array(arr) => {
            yaml_rust2::Yaml::Array(arr.into_iter().map(to_yaml_value_single).collect())
        }
        Value::Table(table) => to_yaml_value(table),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_valid_yaml() {
        let input = "key: value";
        let result = deserialize(input.to_string());
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("key").unwrap(), &Value::String("value".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = deserialize(input.to_string());
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_malformed_yaml() {
        let input = "key: : value"; // Invalid syntax
        let result = deserialize(input.to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse YAML"));
    }

    #[test]
    fn test_multiple_documents() {
        let input = "---\nkey: value\n---\nanother: doc";
        let result = deserialize(input.to_string());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Expected a single YAML document")
        );
    }

    #[test]
    fn test_single_empty_document() {
        let input = "---"; // A single empty document
        let result = deserialize(input.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_non_string_keys() {
        let input = "123: value";
        let result = deserialize(input.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "YAML keys must be strings");
    }

    #[test]
    fn test_deserialize() {
        let yaml_string = r#"---
key: value"#;
        let parsed_map = deserialize(yaml_string.to_string()).unwrap();
        assert_eq!(
            parsed_map,
            Map::from_iter(vec![(
                "key".to_string(),
                Value::String("value".to_string())
            )])
        );
    }
    #[test]
    fn test_deserialize_array() {
        let yaml_string = r#"---
- name: John
- name: Jane"#;
        let parsed_map = deserialize(yaml_string.to_string());
        assert!(parsed_map.is_err());
    }

    #[test]
    fn test_serialize() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let yaml_string = serialize(map);
        assert_eq!(
            yaml_string,
            r#"---
key: value"#
        );
    }

    #[test]
    fn test_serialize_array() {
        let mut map = Map::new();
        map.insert(
            "array".to_string(),
            Value::Array(vec![Value::Int(1), Value::String("two".to_string())]),
        );
        let yaml_string = serialize(map);
        assert_eq!(
            yaml_string,
            r#"---
array:
  - 1
  - two"#
        );
    }

    mod from_yaml_value {
        use super::*;

        #[test]
        fn test_from_null() {
            let yaml_value = yaml_rust2::Yaml::Null;
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::None);
        }

        #[test]
        fn test_from_bool() {
            let yaml_value = yaml_rust2::Yaml::Boolean(true);
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::Bool(true));
        }

        #[test]
        fn test_from_int() {
            let yaml_value = yaml_rust2::Yaml::Integer(42);
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::Int(42));
        }

        #[test]
        fn test_from_float() {
            let yaml_value = yaml_rust2::Yaml::Real("3.1".to_string());
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::Float(3.1));
            let yaml_value = yaml_rust2::Yaml::Real("42".to_string());
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::Int(42));
        }

        #[test]
        fn test_from_string() {
            let yaml_value = yaml_rust2::Yaml::String("Hello".to_string());
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::String("Hello".to_string()));
        }

        #[test]
        fn test_from_array() {
            let yaml_value = yaml_rust2::Yaml::Array(vec![
                yaml_rust2::Yaml::Integer(1),
                yaml_rust2::Yaml::String("two".to_string()),
            ]);
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(
                parsed_value,
                Value::Array(vec![Value::Int(1), Value::String("two".to_string())])
            );
        }

        #[test]
        fn test_from_hash() {
            let yaml_value = yaml_rust2::Yaml::Hash(
                [(
                    yaml_rust2::Yaml::String("key".to_string()),
                    yaml_rust2::Yaml::String("value".to_string()),
                )]
                .iter()
                .cloned()
                .collect(),
            );
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(
                parsed_value,
                Value::Table(Table::from_iter(vec![(
                    "key".to_string(),
                    Value::String("value".to_string())
                )]))
            );
        }

        #[test]
        fn test_from_bad_value() {
            let yaml_value = yaml_rust2::Yaml::BadValue;
            let parsed_value = from_yaml_value(&yaml_value);
            assert_eq!(parsed_value, Value::None);
        }
    }

    mod to_yaml_value {
        use super::*;

        #[test]
        fn test_none_to_yaml_value_single() {
            let value = Value::None;
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(yaml_value, yaml_rust2::Yaml::Null);
        }

        #[test]
        fn test_bool_to_yaml_value_single() {
            let value = Value::Bool(true);
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(yaml_value, yaml_rust2::Yaml::Boolean(true));
        }

        #[test]
        fn test_int_to_yaml_value_single() {
            let value = Value::Int(42);
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(yaml_value, yaml_rust2::Yaml::Integer(42));
        }

        #[test]
        fn test_float_to_yaml_value_single() {
            let value = Value::Float(3.1);
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(yaml_value, yaml_rust2::Yaml::Real("3.1".to_string()));
        }

        #[test]
        fn test_string_to_yaml_value_single() {
            let value = Value::String("Hello".to_string());
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(yaml_value, yaml_rust2::Yaml::String("Hello".to_string()));
        }

        #[test]
        fn test_array_to_yaml_value_single() {
            let value = Value::Array(vec![Value::Int(1), Value::String("two".to_string())]);
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(
                yaml_value,
                yaml_rust2::Yaml::Array(vec![
                    yaml_rust2::Yaml::Integer(1),
                    yaml_rust2::Yaml::String("two".to_string())
                ])
            );
        }

        #[test]
        fn test_hash_to_yaml_value_single() {
            let mut table = Table::new();
            table.insert("key".to_string(), Value::String("value".to_string()));
            let value = Value::Table(table);
            let yaml_value = to_yaml_value_single(value);
            assert_eq!(
                yaml_value,
                yaml_rust2::Yaml::Hash(
                    [(
                        yaml_rust2::Yaml::String("key".to_string()),
                        yaml_rust2::Yaml::String("value".to_string())
                    )]
                    .iter()
                    .cloned()
                    .collect()
                )
            );
        }

        #[test]
        fn test_to_yaml_value() {
            let mut map = Map::new();
            map.insert("key".to_string(), Value::String("value".to_string()));
            let yaml_value = to_yaml_value(map);
            assert_eq!(
                yaml_value,
                yaml_rust2::Yaml::Hash(
                    [(
                        yaml_rust2::Yaml::String("key".to_string()),
                        yaml_rust2::Yaml::String("value".to_string())
                    )]
                    .iter()
                    .cloned()
                    .collect()
                )
            );
        }
    }
}
