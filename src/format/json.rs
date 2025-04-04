use crate::value::{Map, Table, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let json_content: serde_json::Value =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    let mut map = Map::new();
    if let Some(obj) = json_content.as_object() {
        for (key, value) in obj {
            map.insert(key.clone(), from_json_value(value));
        }
    }
    Ok(map)
}

fn from_json_value(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(arr) => {
            let mut values = Vec::new();
            for item in arr {
                values.push(from_json_value(item));
            }
            Value::Array(values)
        }
        serde_json::Value::Object(obj) => {
            let mut table = Table::new();
            for (key, value) in obj {
                table.insert(key.clone(), from_json_value(value));
            }
            Value::Table(table)
        }
    }
}

pub(crate) fn serialize(value: Map<String, Value>) -> String {
    let json_value = to_json_value(value);
    serde_json::to_string(&json_value).unwrap()
}

fn to_json_value(value: Map<String, Value>) -> serde_json::Value {
    serde_json::Value::Object(
        value
            .into_iter()
            .map(|(k, v)| (k, to_json_value_single(v)))
            .collect(),
    )
}

fn to_json_value_single(value: Value) -> serde_json::Value {
    match value {
        Value::None => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Int(i) => serde_json::Value::Number(serde_json::Number::from(i)),
        Value::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(f).unwrap()),
        Value::String(s) => serde_json::Value::String(s),
        Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(to_json_value_single).collect())
        }
        Value::Table(table) => to_json_value(table),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    mod serialize {
        use super::*;

        #[test]
        fn test_serialize() {
            let mut map = Map::new();
            map.insert("key".to_string(), Value::String("value".to_string()));
            let json_string = serialize(map);
            assert_eq!(json_string, r#"{"key":"value"}"#);
        }

        #[test]
        fn test_serialize_array() {
            let mut map = Map::new();
            map.insert(
                "array".to_string(),
                Value::Array(vec![Value::Int(1), Value::String("two".to_string())]),
            );
            let json_string = serialize(map);
            assert_eq!(json_string, r#"{"array":[1,"two"]}"#);
        }
    }

    mod from_json_value {
        use super::*;

        #[test]
        fn test_from_null() {
            let json_value = serde_json::Value::Null;
            let parsed_value = from_json_value(&json_value);
            assert_eq!(parsed_value, Value::None);
        }

        #[test]
        fn test_from_bool() {
            let json_value = serde_json::Value::Bool(true);
            let parsed_value = from_json_value(&json_value);
            assert_eq!(parsed_value, Value::Bool(true));
        }

        #[test]
        fn test_from_int() {
            let json_value = serde_json::Value::Number(serde_json::Number::from(42));
            let parsed_value = from_json_value(&json_value);
            assert_eq!(parsed_value, Value::Int(42));
        }

        #[test]
        fn test_from_float() {
            let json_value = serde_json::Value::Number(serde_json::Number::from_f64(3.1).unwrap());
            let parsed_value = from_json_value(&json_value);
            assert_eq!(parsed_value, Value::Float(3.1));
        }

        #[test]
        fn test_from_string() {
            let json_value = serde_json::Value::String("Hello".to_string());
            let parsed_value = from_json_value(&json_value);
            assert_eq!(parsed_value, Value::String("Hello".to_string()));
        }

        #[test]
        fn test_from_array() {
            let json_value = serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from(1)),
                serde_json::Value::String("two".to_string()),
            ]);
            let parsed_value = from_json_value(&json_value);
            assert_eq!(
                parsed_value,
                Value::Array(vec![Value::Int(1), Value::String("two".to_string())])
            );
        }

        #[test]
        fn test_from_object() {
            let json_value = serde_json::Value::Object(
                [(
                    "key".to_string(),
                    serde_json::Value::String("value".to_string()),
                )]
                .iter()
                .cloned()
                .collect(),
            );
            let parsed_value = from_json_value(&json_value);
            assert_eq!(
                parsed_value,
                Value::Table(Table::from_iter(vec![(
                    "key".to_string(),
                    Value::String("value".to_string())
                )]))
            );
        }
    }

    mod to_json_value {
        use super::*;

        #[test]
        fn test_none_to_json_value_single() {
            let value = Value::None;
            let json_value = to_json_value_single(value);
            assert_eq!(json_value, serde_json::Value::Null);
        }

        #[test]
        fn test_bool_to_json_value_single() {
            let value = Value::Bool(true);
            let json_value = to_json_value_single(value);
            assert_eq!(json_value, serde_json::Value::Bool(true));
        }

        #[test]
        fn test_int_to_json_value_single() {
            let value = Value::Int(42);
            let json_value = to_json_value_single(value);
            assert_eq!(
                json_value,
                serde_json::Value::Number(serde_json::Number::from(42))
            );
        }

        #[test]
        fn test_float_to_json_value_single() {
            let value = Value::Float(3.1);
            let json_value = to_json_value_single(value);
            assert_eq!(
                json_value,
                serde_json::Value::Number(serde_json::Number::from_f64(3.1).unwrap())
            );
        }

        #[test]
        fn test_string_to_json_value_single() {
            let value = Value::String("Hello".to_string());
            let json_value = to_json_value_single(value);
            assert_eq!(json_value, serde_json::Value::String("Hello".to_string()));
        }

        #[test]
        fn test_array_to_json_value_single() {
            let value = Value::Array(vec![Value::Int(1), Value::String("two".to_string())]);
            let json_value = to_json_value_single(value);
            assert_eq!(
                json_value,
                serde_json::Value::Array(vec![
                    serde_json::Value::Number(serde_json::Number::from(1)),
                    serde_json::Value::String("two".to_string())
                ])
            );
        }

        #[test]
        fn test_table_to_json_value_single() {
            let mut table = Table::new();
            table.insert("key".to_string(), Value::String("value".to_string()));
            let value = Value::Table(table);
            let json_value = to_json_value_single(value);
            assert_eq!(
                json_value,
                serde_json::Value::Object(
                    [(
                        "key".to_string(),
                        serde_json::Value::String("value".to_string())
                    )]
                    .iter()
                    .cloned()
                    .collect()
                )
            );
        }

        #[test]
        fn test_to_json_value() {
            let mut map = Map::new();
            map.insert("key".to_string(), Value::String("value".to_string()));
            let json_value = to_json_value(map);
            assert_eq!(
                json_value,
                serde_json::Value::Object(
                    [(
                        "key".to_string(),
                        serde_json::Value::String("value".to_string())
                    )]
                    .iter()
                    .cloned()
                    .collect()
                )
            );
        }
    }
}
