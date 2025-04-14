//! Definition for `Value`

use crate::error::CannotConvert;
use std::convert::{From, TryInto};

/// A type alias for a map that can be either ordered or unordered.
pub(crate) type Map<K, V> = indexmap::IndexMap<K, V>;

/// A type alias for an Array in a config
pub(crate) type Array = Vec<Value>;

/// A type alias for a Table in a config
pub(crate) type Table = Map<String, Value>;

/// A type that represents a value in a configuration file.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
    None,
    Array(Array),
    Table(Table),
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl Value {
    /// Creates a new `Value` from a given variable.
    pub fn new<V>(value: V) -> Self
    where
        V: Into<Value>,
    {
        value.into()
    }

    /// Gets a reference to the value associated with the given key in a table.
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            Value::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Gets a mutable reference to the value associated with the given key in a table.
    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Value::Table(table) => Some(table),
            _ => None,
        }
    }

    /// Gets a reference to the value associated with the given key in a table.
    pub fn as_array(&self) -> Option<&Array> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Gets a mutable reference to the value associated with the given key in a table.
    pub fn as_array_mut(&mut self) -> Option<&mut Array> {
        match self {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Gets a reference to the value associated with the given key in a table.
    pub fn get(&self, key: &str) -> Option<&Value> {
        match self {
            Value::Table(table) => table.get(key),
            _ => None,
        }
    }

    /// Gets a mutable reference to the value associated with the given key in a table.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        match self {
            Value::Table(table) => table.get_mut(key),
            _ => None,
        }
    }

    /// Checks if the value is a table.
    pub fn is_table(&self) -> bool {
        matches!(self, Value::Table(_))
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "null"),
            Value::Array(arr) => {
                let arr_str = arr
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "[{}]", arr_str)
            }
            Value::Table(table) => {
                let table_str = table
                    .iter()
                    .map(|(k, v)| format!("({}: {})", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{{{}}}", table_str)
            }
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Float(n) => write!(f, "{}", n),
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Value::None,
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl TryInto<String> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Value::None => Ok("null".to_string()),
            Value::String(s) => Ok(s),
            Value::Float(n) => Ok(n.to_string()),
            Value::Int(n) => Ok(n.to_string()),
            Value::Array(_) => Err(CannotConvert::new("Array", "String")),
            Value::Table(_) => Err(CannotConvert::new("Table", "String")),
            Value::Bool(b) => Ok(b.to_string()),
        }
    }
}

impl TryInto<f64> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::None => Ok(0.0),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| CannotConvert::new("String", "Float")),
            Value::Float(n) => Ok(n),
            Value::Int(n) => Ok(n as f64),
            Value::Array(_) => Err(CannotConvert::new("Array", "Float")),
            Value::Table(_) => Err(CannotConvert::new("Table", "Float")),
            Value::Bool(b) => Ok(if b { 1.0 } else { 0.0 }),
        }
    }
}

impl TryInto<i64> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::None => Ok(0),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| CannotConvert::new("String", "Int")),
            Value::Float(n) => Ok(n as i64),
            Value::Int(n) => Ok(n),
            Value::Array(_) => Err(CannotConvert::new("Array", "Int")),
            Value::Table(_) => Err(CannotConvert::new("Table", "Int")),
            Value::Bool(b) => Ok(if b { 1 } else { 0 }),
        }
    }
}

impl TryInto<Vec<Value>> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<Vec<Value>, Self::Error> {
        match self {
            Value::None => Ok(vec![]),
            Value::String(_) => Err(CannotConvert::new("String", "Array")),
            Value::Float(_) => Err(CannotConvert::new("Float", "Array")),
            Value::Int(_) => Err(CannotConvert::new("Int", "Array")),
            Value::Array(arr) => Ok(arr),
            Value::Table(_) => Err(CannotConvert::new("Table", "Array")),
            Value::Bool(_) => Err(CannotConvert::new("Bool", "Array")),
        }
    }
}

impl TryInto<Map<String, Value>> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<Map<String, Value>, Self::Error> {
        match self {
            Value::None => Ok(Map::new()),
            Value::String(_) => Err(CannotConvert::new("String", "Table")),
            Value::Float(_) => Err(CannotConvert::new("Float", "Table")),
            Value::Int(_) => Err(CannotConvert::new("Int", "Table")),
            Value::Array(_) => Err(CannotConvert::new("Array", "Table")),
            Value::Table(table) => Ok(table),
            Value::Bool(_) => Err(CannotConvert::new("Bool", "Table")),
        }
    }
}

impl TryInto<bool> for Value {
    type Error = CannotConvert;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::None => Ok(false),
            Value::String(s) => match s.to_lowercase().as_str() {
                "t" | "true" | "True" | "1" => Ok(true),
                _ => Ok(false),
            },
            Value::Float(n) => Ok(n != 0.0),
            Value::Int(n) => Ok(n != 0),
            Value::Array(_) => Err(CannotConvert::new("Array", "Bool")),
            Value::Table(_) => Err(CannotConvert::new("Table", "Bool")),
            Value::Bool(b) => Ok(b),
        }
    }
}

impl From<Map<String, Value>> for Value {
    fn from(value: Map<String, Value>) -> Self {
        Value::Table(value)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(value: &'a str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value as f64)
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Value::Int(value as i64)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Value::Int(value as i64)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Value::Int(value as i64)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::Int(value as i64)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Array(value)
    }
}

impl From<&[Value]> for Value {
    fn from(value: &[Value]) -> Self {
        Value::Array(value.to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_value_new() {
        let value = Value::new(Value::None);
        assert_eq!(value, Value::None);
    }

    #[test]
    fn test_value_get() {
        let value = Value::new(Value::None);
        assert_eq!(value.get("key"), None);
    }

    #[test]
    fn test_value_get_table() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let value = Value::new(Value::Table(map));
        assert_eq!(value.get("key"), Some(&Value::String("value".to_string())));
    }

    #[test]
    fn test_value_get_not_found() {
        let value = Value::new(Value::None);
        assert_eq!(value.get("key"), None);
    }

    #[test]
    fn test_value_is_table() {
        let value = Value::new(Value::None);
        assert!(!value.is_table());
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let value = Value::new(Value::Table(map));
        assert!(value.is_table());
    }

    #[test]
    fn test_value_get_mut() {
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let mut value = Value::new(Value::Table(map));
        assert_eq!(
            value.get_mut("key"),
            Some(&mut Value::String("value".to_string()))
        );
    }

    #[test]
    fn test_value_get_mut_not_found() {
        let mut value = Value::new(Value::None);
        assert_eq!(value.get_mut("key"), None);
    }

    #[test]
    fn test_value_display() {
        let value = Value::String("test".to_string());
        assert_eq!(value.to_string(), "\"test\"");
        let value = Value::Float(1.0);
        assert_eq!(value.to_string(), "1");
        let value = Value::Int(1);
        assert_eq!(value.to_string(), "1");
        let value = Value::Bool(true);
        assert_eq!(value.to_string(), "true");
        let value = Value::Array(vec![Value::String("test".to_string())]);
        assert_eq!(value.to_string(), "[\"test\"]");
        let mut map = Map::new();
        map.insert("key".to_string(), Value::String("value".to_string()));
        let value = Value::Table(map);
        assert_eq!(value.to_string(), "{(key: \"value\")}");
        let value = Value::None;
        assert_eq!(value.to_string(), "null");
    }

    mod value_from {
        use super::*;

        fn test_value_from<T: Into<Value>>(value: T, expected: Value)
        where
            Value: std::convert::From<T>,
        {
            let result = Value::from(value);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_value_from_bool() {
            let value = true;
            let expected = Value::Bool(true);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_map() {
            let value = Map::new();
            let expected = Value::Table(value.clone());
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_string() {
            let value = "test".to_string();
            let expected = Value::String("test".to_string());
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_str() {
            let value = "test";
            let expected = Value::String("test".to_string());
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_f64() {
            let value: f64 = 1.0;
            let expected = Value::Float(1.0);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_f32() {
            let value: f32 = 1.0;
            let expected = Value::Float(1.0);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_i128() {
            let value: i128 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_i64() {
            let value: i64 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_i32() {
            let value: i32 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_i16() {
            let value: i16 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_i8() {
            let value: i8 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_u128() {
            let value: u128 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_u64() {
            let value: u64 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_u32() {
            let value: u32 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_u16() {
            let value: u16 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_u8() {
            let value: u8 = 1;
            let expected = Value::Int(1);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_vec() {
            let value = vec![Value::String("test".to_string())];
            let expected = Value::Array(vec![Value::String("test".to_string())]);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_array() {
            let value: &[Value] = &[Value::String("test".to_string())];
            let expected = Value::Array(vec![Value::String("test".to_string())]);
            test_value_from(value, expected);
        }

        #[test]
        fn test_value_from_none() {
            let value: Option<String> = None;
            let expected = Value::None;
            test_value_from(value, expected);
            let value: Option<String> = Some("test".to_string());
            let expected = Value::String("test".to_string());
            test_value_from(value, expected);
        }
    }

    mod value_try_into {
        use super::*;

        #[test]
        fn test_value_try_into_string() {
            let value = Value::String("test".to_string());
            let result: Result<String, CannotConvert> = value.try_into();
            assert_ne!(result, Err(CannotConvert::new("String", "String")));
            assert_eq!(result, Ok("test".to_string()));

            let value = Value::None;
            let result: Result<String, CannotConvert> = value.try_into();
            assert_ne!(result, Err(CannotConvert::new("None", "String")));
            assert_eq!(result, Ok("null".to_string()));

            let value = Value::Array(vec![]);
            let result: Result<String, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Array", "String")));
            assert_ne!(result, Ok("".to_string()));

            let value = Value::Float(1.0);
            let result: Result<String, CannotConvert> = value.try_into();
            assert_eq!(result, Ok("1".to_string()));
            assert_ne!(result, Err(CannotConvert::new("Float", "String")));

            let value = Value::Int(42);
            let result: Result<String, CannotConvert> = value.try_into();
            assert_eq!(result, Ok("42".to_string()));
            assert_ne!(result, Err(CannotConvert::new("Int", "String")));

            let value = Value::Bool(true);
            let result: Result<String, CannotConvert> = value.try_into();
            assert_eq!(result, Ok("true".to_string()));
            assert_ne!(result, Err(CannotConvert::new("Bool", "String")));

            let value = Value::Table(Map::new());
            let result: Result<String, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Table", "String")));
            assert_ne!(result, Ok("".to_string()));
        }

        #[test]
        fn test_value_try_into_f64() {
            let value = Value::Float(1.0);
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1.0));

            let value = Value::String("1.0".to_string());
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1.0));
            let value = Value::String("1y".to_string());
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("String", "Float")));

            let value = Value::None;
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(0.0));

            let value = Value::Array(vec![]);
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Array", "Float")));

            let value = Value::Int(42);
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(42.0));

            let value = Value::Bool(true);
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1.0));
            let value = Value::Bool(false);
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(0.0));

            let value = Value::Table(Map::new());
            let result: Result<f64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Table", "Float")));
        }

        #[test]
        fn test_value_try_into_i64() {
            let value = Value::Int(1);
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1));

            let value = Value::String("1".to_string());
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1));
            let value = Value::String("1y".to_string());
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("String", "Int")));

            let value = Value::None;
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(0));

            let value = Value::Array(vec![]);
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Array", "Int")));

            let value = Value::Float(42.0);
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(42));

            let value = Value::Bool(true);
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(1));
            let value = Value::Bool(false);
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(0));

            let value = Value::Table(Map::new());
            let result: Result<i64, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Table", "Int")));
        }

        #[test]
        fn test_value_try_into_bool() {
            let value = Value::String("true".to_string());
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(true));
            let value = Value::String("True".to_string());
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(true));
            let value = Value::String("false".to_string());
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(false));

            let value = Value::None;
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(false));

            let value = Value::Array(vec![]);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Array", "Bool")));

            let value = Value::Float(1.0);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(true));
            let value = Value::Float(0.0);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(false));

            let value = Value::Int(1);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(true));
            let value = Value::Int(0);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(false));

            let value = Value::Bool(true);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(true));
            let value = Value::Bool(false);
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(false));

            let value = Value::Table(Map::new());
            let result: Result<bool, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Table", "Bool")));
        }

        #[test]
        fn test_value_try_into_vec() {
            let value = Value::Array(vec![Value::String("test".to_string())]);
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(vec![Value::String("test".to_string())]));

            let value = Value::None;
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(vec![]));

            let value = Value::String("test".to_string());
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("String", "Array")));

            let value = Value::Table(Map::new());
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Table", "Array")));

            let value = Value::Bool(true);
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Bool", "Array")));

            let value = Value::Float(1.0);
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Float", "Array")));

            let value = Value::Int(1);
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Int", "Array")));
        }

        #[test]
        fn test_value_try_into_array() {
            let value = Value::Array(vec![Value::String("test".to_string())]);
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(vec![Value::String("test".to_string())]));

            let value = Value::None;
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(vec![]));

            let value = Value::String("test".to_string());
            let result: Result<Vec<Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("String", "Array")));
        }

        #[test]
        fn test_value_try_into_map() {
            let value = Value::Table(Map::new());
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(Map::new()));

            let value = Value::None;
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Ok(Map::new()));

            let value = Value::String("test".to_string());
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("String", "Table")));

            let value = Value::Array(vec![]);
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Array", "Table")));

            let value = Value::Bool(true);
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Bool", "Table")));

            let value = Value::Float(3.1);
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Float", "Table")));

            let value = Value::Int(1);
            let result: Result<Map<String, Value>, CannotConvert> = value.try_into();
            assert_eq!(result, Err(CannotConvert::new("Int", "Table")));
        }
    }
}
