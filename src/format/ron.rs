use crate::value::{Map, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let parsed_value: ron::Value = ron::from_str(&content).map_err(|e| e.to_string())?;
    let mut map = Map::new();
    match parsed_value {
        ron::Value::Map(m) => {
            for (key, value) in m {
                map.insert(check_key(key), from_ron_value(value));
            }
        }
        _ => panic!("Expected a RON map"),
    }
    Ok(map)
}

fn from_ron_value(value: ron::Value) -> Value {
    match value {
        ron::Value::Char(c) => Value::String(c.to_string()),
        ron::Value::String(s) => Value::String(s),
        ron::Value::Bytes(b) => Value::String(String::from_utf8_lossy(&b).to_string()),
        ron::Value::Number(n) => {
            let float = n.into_f64();
            if float.fract() == 0.0 {
                Value::Int(float as i64)
            } else {
                Value::Float(float)
            }
        }
        ron::Value::Option(o) => match o {
            Some(v) => from_ron_value(*v),
            None => Value::None,
        },
        ron::Value::Bool(b) => Value::Bool(b),
        ron::Value::Seq(s) => {
            let mut values = Vec::new();
            for item in s {
                values.push(from_ron_value(item));
            }
            Value::Array(values)
        }
        ron::Value::Map(map) => {
            let mut new_map = Map::new();
            for (key, value) in map {
                new_map.insert(check_key(key), from_ron_value(value));
            }
            Value::Table(new_map)
        }
        ron::Value::Unit => Value::None,
    }
}

fn check_key(key: ron::Value) -> String {
    match key {
        ron::Value::String(s) => s,
        _ => panic!("Invalid key type in RON map"),
    }
}

pub(crate) fn serialize(value: Map<String, Value>) -> String {
    let mut ron_map = ron::Map::new();
    for (key, value) in value {
        ron_map.insert(key, to_ron_value(value.clone()));
    }
    ron::to_string(&ron_map).unwrap()
}

fn to_ron_value(value: Value) -> ron::Value {
    match value {
        Value::String(s) => ron::Value::String(s),
        Value::Int(i) => {
            if let Ok(i32_value) = i.try_into() {
                ron::Value::Number(ron::Number::I32(i32_value))
            } else {
                ron::Value::Number(ron::Number::I64(i))
            }
        }
        Value::Float(f) => ron::Value::Number(ron::Number::from(f)),
        Value::Bool(b) => ron::Value::Bool(b),
        Value::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(to_ron_value(item));
            }
            ron::Value::Seq(values)
        }
        Value::Table(table) => {
            let mut ron_map = ron::Map::new();
            for (key, value) in table {
                ron_map.insert(key, to_ron_value(value));
            }
            ron::Value::Map(ron_map)
        }
        _ => panic!("Unsupported value type for RON serialization"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_check_key() {
        let key = ron::Value::String("key".to_string());
        let result = check_key(key);
        assert_eq!(result, "key");
    }

    #[test]
    #[should_panic]
    fn test_check_key_not_string() {
        let key = ron::Value::Number(ron::Number::from(42));
        let _result = check_key(key);
    }

    #[test]
    fn test_invalid() {
        let ron_content = r#"[section"#;
        let result = deserialize(ron_content.to_string());
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_expected_ron_map() {
        let non_map_ron = r#""string_value""#; // Not a map, should panic
        let _result = deserialize(non_map_ron.to_string());
    }

    #[test]
    fn test_serialize() {
        let map = Map::from_iter(vec![
            ("key1".to_string(), Value::String("value1".to_string())),
            ("key2".to_string(), Value::Int(42)),
        ]);
        let serialized = serialize(map);
        assert!(serialized.contains("key1"));
        assert!(serialized.contains("value1"));
        assert!(serialized.contains("key2"));
        assert!(serialized.contains("42"));
    }

    #[test]
    fn test_deserialize() {
        let ron_content = r#"
            (
                key1: "value1",
                key2: 42,
            )
            "#;
        let parsed_map = deserialize(ron_content.to_string()).unwrap();
        assert_eq!(
            parsed_map,
            Map::from_iter(vec![
                ("key1".to_string(), Value::String("value1".to_string())),
                ("key2".to_string(), Value::Int(42)),
            ])
        );
    }

    mod from_ron_value {
        use super::*;

        #[test]
        fn test_from_null() {
            let ron_value = ron::Value::Unit;
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::None);
        }

        #[test]
        fn test_from_char() {
            let ron_value = ron::Value::Char('c');
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::String("c".to_string()));
        }

        #[test]
        fn test_from_string() {
            let ron_value = ron::Value::String("value".to_string());
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::String("value".to_string()));
        }

        #[test]
        fn test_from_int() {
            let ron_value = ron::Value::Number(ron::Number::from(42));
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::Int(42));
        }

        #[test]
        fn test_from_float() {
            let ron_value = ron::Value::Number(ron::Number::from(3.1));
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::Float(3.1));
        }

        #[test]
        fn test_from_bool() {
            let ron_value = ron::Value::Bool(true);
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::Bool(true));
        }

        #[test]
        fn test_from_array() {
            let ron_value = ron::Value::Seq(vec![
                ron::Value::Number(ron::Number::from(1)),
                ron::Value::String("two".to_string()),
            ]);
            let value = from_ron_value(ron_value);
            assert_eq!(
                value,
                Value::Array(vec![Value::Int(1), Value::String("two".to_string())])
            );
        }

        #[test]
        fn test_from_bytes() {
            let ron_value = ron::Value::Bytes(vec![1, 2, 3]);
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::String("\u{1}\u{2}\u{3}".to_string()));
        }

        #[test]
        fn test_from_map() {
            let ron_value = ron::Value::Map(ron::Map::from_iter(vec![(
                "key".to_string(),
                ron::Value::String("value".to_string()),
            )]));
            let value = from_ron_value(ron_value);
            assert_eq!(
                value,
                Value::Table(Map::from_iter(vec![(
                    "key".to_string(),
                    Value::String("value".to_string())
                )]))
            );
        }

        #[test]
        fn test_from_option() {
            let ron_value =
                ron::Value::Option(Some(Box::new(ron::Value::String("value".to_string()))));
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::String("value".to_string()));
        }

        #[test]
        fn test_from_option_none() {
            let ron_value = ron::Value::Option(None);
            let value = from_ron_value(ron_value);
            assert_eq!(value, Value::None);
        }
    }

    mod to_ron_value {
        use super::*;

        #[test]
        fn test_bool_to_ron_value() {
            let value = Value::Bool(true);
            let ron_value = to_ron_value(value);
            assert_eq!(ron_value, ron::Value::Bool(true));
        }

        #[test]
        fn test_string_to_ron_value() {
            let value = Value::String("value".to_string());
            let ron_value = to_ron_value(value);
            assert_eq!(ron_value, ron::Value::String("value".to_string()));
        }

        #[test]
        fn test_float_to_ron_value() {
            let value = Value::Float(3.1);
            let ron_value = to_ron_value(value);
            assert_eq!(ron_value, ron::Value::Number(ron::Number::from(3.1)));
        }

        #[test]
        fn test_array_to_ron_value() {
            let value = Value::Array(vec![Value::Int(1), Value::String("two".to_string())]);
            let ron_value = to_ron_value(value);
            assert_eq!(
                ron_value,
                ron::Value::Seq(vec![
                    ron::Value::Number(ron::Number::from(1)),
                    ron::Value::String("two".to_string())
                ])
            );
        }

        #[test]
        fn test_i64_to_ron_value() {
            let value = Value::Int(4200000000);
            let _ron_value = to_ron_value(value);
        }

        #[test]
        fn test_table_to_ron_value() {
            let value = Value::Table(Map::from_iter(vec![(
                "key".to_string(),
                Value::String("value".to_string()),
            )]));
            let ron_value = to_ron_value(value);
            assert_eq!(
                ron_value,
                ron::Value::Map(ron::Map::from_iter(vec![(
                    "key".to_string(),
                    ron::Value::String("value".to_string())
                )]))
            );
        }

        #[test]
        fn test_unsupported_value() {
            let value = Value::None;
            let result = std::panic::catch_unwind(|| {
                to_ron_value(value);
            });
            assert!(result.is_err());
        }
    }
}
