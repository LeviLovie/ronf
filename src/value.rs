use crate::error::CannotConvert;
use serde::de::{DeserializeSeed, Deserializer, EnumAccess, VariantAccess, Visitor};
use std::convert::{From, TryInto};

#[cfg(feature = "ordered")]
pub(crate) type Map<K, V> = indexmap::IndexMap<K, V>;
#[cfg(not(feature = "ordered"))]
pub(crate) type Map<K, V> = std::collections::HashMap<K, V>;

pub(crate) type Array = Vec<Value>;
pub(crate) type Table = Map<String, Value>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    Array(Array),
    Table(Table),
    String(String),
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl Value {
    pub fn new<V>(value: V) -> Self
    where
        V: Into<Value>,
    {
        value.into()
    }
}

struct ValueDeserializer {
    value: Value,
}

impl ValueDeserializer {
    fn new(value: Value) -> Self {
        ValueDeserializer { value }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => visitor.visit_string(s),
            Value::Float(n) => visitor.visit_f64(n),
            Value::Int(n) => visitor.visit_i64(n),
            Value::Table(t) => visitor.visit_map(TableDeserializer::new(t)),
            Value::Array(arr) => visitor.visit_seq(ArrayDeserializer::new(arr)),
            Value::Bool(b) => visitor.visit_bool(b),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<bool>()
                .map_err(|_| serde::de::Error::custom("invalid boolean"))
                .and_then(|b| visitor.visit_bool(b)),
            Value::Float(n) => visitor.visit_bool(n != 0.0),
            Value::Int(n) => visitor.visit_bool(n != 0),
            Value::Bool(b) => visitor.visit_bool(b),
            _ => Err(serde::de::Error::custom("expected a boolean")),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<i8>()
                .map_err(|_| serde::de::Error::custom("invalid i8"))
                .and_then(|n| visitor.visit_i8(n)),
            Value::Float(n) => visitor.visit_i8(n as i8),
            Value::Int(n) => visitor.visit_i8(n as i8),
            _ => Err(serde::de::Error::custom("expected an i8")),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<i16>()
                .map_err(|_| serde::de::Error::custom("invalid i16"))
                .and_then(|n| visitor.visit_i16(n)),
            Value::Float(n) => visitor.visit_i16(n as i16),
            Value::Int(n) => visitor.visit_i16(n as i16),
            _ => Err(serde::de::Error::custom("expected an i16")),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<i32>()
                .map_err(|_| serde::de::Error::custom("invalid i32"))
                .and_then(|n| visitor.visit_i32(n)),
            Value::Float(n) => visitor.visit_i32(n as i32),
            Value::Int(n) => visitor.visit_i32(n as i32),
            _ => Err(serde::de::Error::custom("expected an i32")),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<i64>()
                .map_err(|_| serde::de::Error::custom("invalid i64"))
                .and_then(|n| visitor.visit_i64(n)),
            Value::Float(n) => visitor.visit_i64(n as i64),
            Value::Int(n) => visitor.visit_i64(n),
            _ => Err(serde::de::Error::custom("expected an i64")),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<u8>()
                .map_err(|_| serde::de::Error::custom("invalid u8"))
                .and_then(|n| visitor.visit_u8(n)),
            Value::Float(n) => visitor.visit_u8(n as u8),
            Value::Int(n) => visitor.visit_u8(n as u8),
            _ => Err(serde::de::Error::custom("expected an u8")),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<u16>()
                .map_err(|_| serde::de::Error::custom("invalid u16"))
                .and_then(|n| visitor.visit_u16(n)),
            Value::Float(n) => visitor.visit_u16(n as u16),
            Value::Int(n) => visitor.visit_u16(n as u16),
            _ => Err(serde::de::Error::custom("expected an u16")),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<u32>()
                .map_err(|_| serde::de::Error::custom("invalid u32"))
                .and_then(|n| visitor.visit_u32(n)),
            Value::Float(n) => visitor.visit_u32(n as u32),
            Value::Int(n) => visitor.visit_u32(n as u32),
            _ => Err(serde::de::Error::custom("expected an u32")),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<u64>()
                .map_err(|_| serde::de::Error::custom("invalid u64"))
                .and_then(|n| visitor.visit_u64(n)),
            Value::Float(n) => visitor.visit_u64(n as u64),
            Value::Int(n) => visitor.visit_u64(n as u64),
            _ => Err(serde::de::Error::custom("expected an u64")),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<f32>()
                .map_err(|_| serde::de::Error::custom("invalid f32"))
                .and_then(|n| visitor.visit_f32(n)),
            Value::Float(n) => visitor.visit_f32(n as f32),
            Value::Int(n) => visitor.visit_f32(n as f32),
            _ => Err(serde::de::Error::custom("expected an f32")),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| serde::de::Error::custom("invalid f64"))
                .and_then(|n| visitor.visit_f64(n)),
            Value::Float(n) => visitor.visit_f64(n),
            Value::Int(n) => visitor.visit_f64(n as f64),
            _ => Err(serde::de::Error::custom("expected an f64")),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => s
                .chars()
                .next()
                .ok_or_else(|| serde::de::Error::custom("invalid char"))
                .and_then(|c| visitor.visit_char(c)),
            _ => Err(serde::de::Error::custom("expected a char")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => visitor.visit_string(s),
            _ => Err(serde::de::Error::custom("expected a string")),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => visitor.visit_string(s),
            _ => Err(serde::de::Error::custom("expected a string")),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => visitor.visit_bytes(s.as_bytes()),
            _ => Err(serde::de::Error::custom("expected bytes")),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            Value::String(s) => visitor.visit_byte_buf(s.into_bytes()),
            _ => Err(serde::de::Error::custom("expected byte buffer")),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            _ => visitor.visit_some(ValueDeserializer::new(self.value)),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_unit(),
            _ => Err(serde::de::Error::custom("expected a unit")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            _ => Err(serde::de::Error::custom("expected a unit struct")),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            _ => visitor.visit_newtype_struct(ValueDeserializer::new(self.value)),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(array) => visitor.visit_seq(ArrayDeserializer::new(array)),
            _ => Err(serde::de::Error::custom("expected a tuple")),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(array) => visitor.visit_seq(ArrayDeserializer::new(array)),
            _ => Err(serde::de::Error::custom("expected a tuple struct")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Table(table) => visitor.visit_map(TableDeserializer::new(table)),
            _ => Err(serde::de::Error::custom("expected a struct")),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(_) | Value::Int(_) | Value::Table(_) => {
                visitor.visit_enum(ValueEnumDeserializer::new(self.value))
            }
            _ => Err(serde::de::Error::custom(
                "expected string, integer, or table for enum",
            )),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(s) => visitor.visit_string(s),
            _ => Err(serde::de::Error::custom("expected an identifier")),
        }
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::None => visitor.visit_none(),
            _ => Err(serde::de::Error::custom("expected ignored any")),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(array) => visitor.visit_seq(ArrayDeserializer::new(array)),
            _ => Err(serde::de::Error::custom("expected an array")),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Table(table) => visitor.visit_map(TableDeserializer::new(table)),
            _ => Err(serde::de::Error::custom("expected a table")),
        }
    }
}

struct ArrayDeserializer {
    array: Array,
    index: usize,
}

impl ArrayDeserializer {
    fn new(array: Array) -> Self {
        ArrayDeserializer { array, index: 0 }
    }
}

impl<'de> serde::de::SeqAccess<'de> for ArrayDeserializer {
    type Error = serde::de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= self.array.len() {
            return Ok(None);
        }

        let value = self.array[self.index].clone();
        self.index += 1;

        seed.deserialize(ValueDeserializer::new(value)).map(Some)
    }
}

struct TableDeserializer {
    table: Table,
    keys: Vec<String>,
    index: usize,
}

impl TableDeserializer {
    fn new(table: Table) -> Self {
        let keys = table.keys().cloned().collect();
        TableDeserializer {
            table,
            keys,
            index: 0,
        }
    }
}

struct ValueEnumDeserializer {
    value: Value,
}

impl ValueEnumDeserializer {
    fn new(value: Value) -> Self {
        ValueEnumDeserializer { value }
    }
}

impl<'de> EnumAccess<'de> for ValueEnumDeserializer {
    type Error = serde::de::value::Error;
    type Variant = ValueVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value {
            Value::String(s) => {
                let string_deserializer = serde::de::value::StringDeserializer::new(s.clone());
                let variant = seed.deserialize(string_deserializer)?;
                Ok((variant, ValueVariantDeserializer::new(Value::None)))
            }
            Value::Int(i) => {
                let u32_value = i as u32;
                let u32_deserializer = serde::de::value::U32Deserializer::new(u32_value);
                let variant = seed.deserialize(u32_deserializer)?;
                Ok((variant, ValueVariantDeserializer::new(Value::None)))
            }
            Value::Bool(b) => {
                let bool_deserializer = serde::de::value::BoolDeserializer::new(b);
                let variant = seed.deserialize(bool_deserializer)?;
                Ok((variant, ValueVariantDeserializer::new(Value::None)))
            }
            Value::Table(t) => {
                if let Some(tag_value) = t.get("tag") {
                    if let Value::String(tag) = tag_value {
                        let string_deserializer =
                            serde::de::value::StringDeserializer::new(tag.clone());
                        let variant = seed.deserialize(string_deserializer)?;

                        // Pass the content of the table for the variant value
                        let content = t.get("content").cloned().unwrap_or(Value::None);
                        Ok((variant, ValueVariantDeserializer::new(content)))
                    } else {
                        Err(serde::de::Error::custom(
                            "expected string tag in enum table",
                        ))
                    }
                } else {
                    Err(serde::de::Error::custom(
                        "expected table with tag field for enum",
                    ))
                }
            }
            _ => Err(serde::de::Error::custom(
                "expected string, integer, or table for enum",
            )),
        }
    }
}

struct ValueVariantDeserializer {
    value: Value,
}

impl ValueVariantDeserializer {
    fn new(value: Value) -> Self {
        ValueVariantDeserializer { value }
    }
}

impl<'de> VariantAccess<'de> for ValueVariantDeserializer {
    type Error = serde::de::value::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(ValueDeserializer::new(self.value))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Array(array) => visitor.visit_seq(ArrayDeserializer::new(array)),
            _ => Err(serde::de::Error::custom("expected array for tuple variant")),
        }
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Table(table) => visitor.visit_map(TableDeserializer::new(table)),
            _ => Err(serde::de::Error::custom(
                "expected table for struct variant",
            )),
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for TableDeserializer {
    type Error = serde::de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.index >= self.keys.len() {
            return Ok(None);
        }

        let key = self.keys[self.index].clone();

        seed.deserialize(ValueDeserializer::new(Value::String(key)))
            .map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let key = &self.keys[self.index];
        self.index += 1;

        let value = self
            .table
            .get(key)
            .ok_or_else(|| serde::de::Error::custom(format!("missing value for key {}", key)))?
            .clone();

        seed.deserialize(ValueDeserializer::new(value))
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::None
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

impl From<std::collections::HashMap<std::string::String, Value>> for Value {
    fn from(value: std::collections::HashMap<std::string::String, Value>) -> Self {
        #[cfg(feature = "ordered")]
        let value = value
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect::<Map<String, Value>>();
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
    fn test_value_from_none() {
        let value: Option<String> = None;
        let expected = Value::None;
        test_value_from(value, expected);
    }
}

#[cfg(test)]
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
    }

    #[test]
    fn test_value_try_into_f64() {
        let value = Value::Float(1.0);
        let result: Result<f64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(1.0));

        let value = Value::String("1.0".to_string());
        let result: Result<f64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(1.0));

        let value = Value::None;
        let result: Result<f64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(0.0));

        let value = Value::Array(vec![]);
        let result: Result<f64, CannotConvert> = value.try_into();
        assert_eq!(result, Err(CannotConvert::new("Array", "Float")));
    }

    #[test]
    fn test_value_try_into_i64() {
        let value = Value::Int(1);
        let result: Result<i64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(1));

        let value = Value::String("1".to_string());
        let result: Result<i64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(1));

        let value = Value::None;
        let result: Result<i64, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(0));

        let value = Value::Array(vec![]);
        let result: Result<i64, CannotConvert> = value.try_into();
        assert_eq!(result, Err(CannotConvert::new("Array", "Int")));
    }

    #[test]
    fn test_value_try_into_bool() {
        let value = Value::String("true".to_string());
        let result: Result<bool, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(true));

        let value = Value::None;
        let result: Result<bool, CannotConvert> = value.try_into();
        assert_eq!(result, Ok(false));

        let value = Value::Array(vec![]);
        let result: Result<bool, CannotConvert> = value.try_into();
        assert_eq!(result, Err(CannotConvert::new("Array", "Bool")));
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
}
