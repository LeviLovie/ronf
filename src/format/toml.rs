use crate::value::{Map, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let table = content.parse::<toml::Table>().map_err(|e| e.to_string())?;
    let mut map = Map::new();
    for (key, value) in table {
        map.insert(key, from_toml_value(&value));
    }
    Ok(map)
}

fn from_toml_value(value: &toml::Value) -> Value {
    match value {
        toml::Value::String(s) => Value::String(s.clone()),
        toml::Value::Integer(i) => Value::Int(*i),
        toml::Value::Float(f) => Value::Float(*f),
        toml::Value::Boolean(b) => Value::Bool(*b),
        toml::Value::Datetime(dt) => Value::String(dt.to_string()),
        toml::Value::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(from_toml_value(item));
            }
            Value::Array(values)
        }
        toml::Value::Table(table) => {
            let mut map = Map::new();
            for (key, value) in table {
                map.insert(key.clone(), from_toml_value(value));
            }
            Value::Table(map)
        }
    }
}

pub(crate) fn serialize(value: Map<String, Value>) -> String {
    let mut table = toml::Table::new();
    for (key, value) in value {
        table.insert(key, to_toml_value(value));
    }
    toml::to_string(&table).unwrap()
}

fn to_toml_value(value: Value) -> toml::Value {
    match value {
        Value::String(s) => toml::Value::String(s),
        Value::Int(i) => toml::Value::Integer(i),
        Value::Float(f) => toml::Value::Float(f),
        Value::Bool(b) => toml::Value::Boolean(b),
        Value::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(to_toml_value(item));
            }
            toml::Value::Array(values)
        }
        Value::Table(table) => {
            let mut toml_table = toml::Table::new();
            for (key, value) in table {
                toml_table.insert(key, to_toml_value(value));
            }
            toml::Value::Table(toml_table)
        }
        _ => panic!("Unsupported value type for TOML serialization"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {
        let toml_content = r#"
            key = "value"
            int_key = 42
            float_key = 3.1
            bool_key = true
            array_key = [1, 2, 3]
            table_key = { nested_key = "nested_value" }
            "#;
        let parsed_map = deserialize(toml_content.to_string()).unwrap();
        assert_eq!(
            parsed_map.get("key").unwrap(),
            &Value::String("value".to_string())
        );
        assert_eq!(parsed_map.get("int_key").unwrap(), &Value::Int(42));
        assert_eq!(parsed_map.get("float_key").unwrap(), &Value::Float(3.1));
        assert_eq!(parsed_map.get("bool_key").unwrap(), &Value::Bool(true));
        assert_eq!(
            parsed_map.get("array_key").unwrap(),
            &Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
        if let Value::Table(table) = parsed_map.get("table_key").unwrap() {
            assert_eq!(
                table.get("nested_key").unwrap(),
                &Value::String("nested_value".to_string())
            );
        } else {
            panic!("Expected a table for 'table_key'");
        }
    }

    #[test]
    fn test_desetialize_section() {
        let toml_content = r#"
            [section]
            key = "value"
            int_key = 42
            "#;
        let parsed_map = deserialize(toml_content.to_string()).unwrap();
        if let Value::Table(table) = parsed_map.get("section").unwrap() {
            assert_eq!(
                table.get("key").unwrap(),
                &Value::String("value".to_string())
            );
            assert_eq!(table.get("int_key").unwrap(), &Value::Int(42));
        } else {
            panic!("Expected a table for 'section'");
        }
    }

    #[test]
    fn test_serialize() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        map.insert("int_key".to_string(), Value::Int(42));
        map.insert("float_key".to_string(), Value::Float(3.1));
        map.insert("bool_key".to_string(), Value::Bool(true));
        let serialized = serialize(map);
        assert!(serialized.contains("key = \"value\""));
        assert!(serialized.contains("int_key = 42"));
        assert!(serialized.contains("float_key = 3.1"));
        assert!(serialized.contains("bool_key = true"));
    }

    #[test]
    fn test_serialize_array() {
        let mut map = Map::new();
        map.insert(
            "array_key".to_string(),
            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        );
        let serialized = serialize(map);
        assert!(serialized.contains("array_key = [1, 2, 3]"));
    }

    mod from_toml_value {
        use super::*;

        #[test]
        fn test_from_toml_value() {
            let toml_value = toml::Value::String("value".to_string());
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(parsed_value, Value::String("value".to_string()));
        }

        #[test]
        fn test_from_toml_array() {
            let toml_value = toml::Value::Array(vec![
                toml::Value::Integer(1),
                toml::Value::String("two".to_string()),
            ]);
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(
                parsed_value,
                Value::Array(vec![Value::Int(1), Value::String("two".to_string())])
            );
        }

        #[test]
        fn test_from_toml_table() {
            let toml_value = toml::Value::Table(toml::Table::new());
            let parsed_value = from_toml_value(&toml_value);
            assert!(matches!(parsed_value, Value::Table(_)));
        }

        #[test]
        fn test_from_toml_bool() {
            let toml_value = toml::Value::Boolean(true);
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(parsed_value, Value::Bool(true));
        }

        #[test]
        fn test_from_toml_integer() {
            let toml_value = toml::Value::Integer(42);
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(parsed_value, Value::Int(42));
        }

        #[test]
        fn test_from_toml_float() {
            let toml_value = toml::Value::Float(3.1);
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(parsed_value, Value::Float(3.1));
        }

        #[test]
        fn test_from_toml_string() {
            let toml_value = toml::Value::String("Hello".to_string());
            let parsed_value = from_toml_value(&toml_value);
            assert_eq!(parsed_value, Value::String("Hello".to_string()));
        }
    }

    mod to_toml_value {
        use super::*;

        #[test]
        fn test_to_toml_value() {
            let value = Value::String("value".to_string());
            let toml_value = to_toml_value(value);
            assert_eq!(toml_value, toml::Value::String("value".to_string()));
        }

        #[test]
        fn test_to_toml_array() {
            let value = Value::Array(vec![Value::Int(1), Value::String("two".to_string())]);
            let toml_value = to_toml_value(value);
            assert_eq!(
                toml_value,
                toml::Value::Array(vec![
                    toml::Value::Integer(1),
                    toml::Value::String("two".to_string())
                ])
            );
        }

        #[test]
        fn test_to_toml_table() {
            let mut table = Map::new();
            table.insert("key".to_string(), Value::String("value".to_string()));
            let value = Value::Table(table);
            let toml_value = to_toml_value(value);
            assert!(matches!(toml_value, toml::Value::Table(_)));
        }

        #[test]
        fn test_to_toml_bool() {
            let value = Value::Bool(true);
            let toml_value = to_toml_value(value);
            assert_eq!(toml_value, toml::Value::Boolean(true));
        }

        #[test]
        fn test_to_toml_integer() {
            let value = Value::Int(42);
            let toml_value = to_toml_value(value);
            assert_eq!(toml_value, toml::Value::Integer(42));
        }

        #[test]
        fn test_to_toml_float() {
            let value = Value::Float(3.1);
            let toml_value = to_toml_value(value);
            assert_eq!(toml_value, toml::Value::Float(3.1));
        }

        #[test]
        fn test_to_toml_string() {
            let value = Value::String("Hello".to_string());
            let toml_value = to_toml_value(value);
            assert_eq!(toml_value, toml::Value::String("Hello".to_string()));
        }

        #[test]
        fn test_to_toml_unsupported() {
            let value = Value::None;
            let result = std::panic::catch_unwind(|| to_toml_value(value));
            assert!(result.is_err());
        }
    }
}
