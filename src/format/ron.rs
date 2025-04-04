use crate::value::{Map, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let parsed_value: ron::Value = ron::from_str(&content).map_err(|e| e.to_string())?;
    let mut map = Map::new();
    match parsed_value {
        ron::Value::Map(m) => {
            for (key, value) in m {
                let key = match key {
                    ron::Value::Char(c) => c.to_string(),
                    ron::Value::String(s) => s,
                    ron::Value::Bytes(b) => String::from_utf8_lossy(&b).to_string(),
                    ron::Value::Number(n) => n.into_f64().to_string(),
                    _ => panic!("Invalid key type in RON map"),
                };
                map.insert(key, from_ron_value(value));
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
                let key = match key {
                    ron::Value::Char(c) => c.to_string(),
                    ron::Value::String(s) => s,
                    ron::Value::Bytes(b) => String::from_utf8_lossy(&b).to_string(),
                    ron::Value::Number(n) => n.into_f64().to_string(),
                    _ => panic!("Invalid key type in RON map"),
                };
                new_map.insert(key, from_ron_value(value));
            }
            Value::Table(new_map)
        }
        ron::Value::Unit => Value::None,
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
        Value::Int(i) => ron::Value::Number(ron::Number::from(i)),
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
    }
}
