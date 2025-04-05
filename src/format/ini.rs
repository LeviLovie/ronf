use crate::value::{Map, Table, Value};

pub(crate) fn deserialize(content: String) -> Result<Map<String, Value>, String> {
    let mut map = Map::new();
    let ini = ini::Ini::load_from_str(&content).map_err(|e| e.to_string())?;
    for (sec, prop) in ini.iter() {
        match sec {
            Some(section) => {
                let mut table = Table::new();
                for (key, value) in prop.iter() {
                    table.insert(
                        key.to_string().to_string(),
                        Value::String(value.to_string()),
                    );
                }
                map.insert(section.to_string(), Value::Table(table));
            }
            None => {
                for (key, value) in prop.iter() {
                    map.insert(key.to_string(), Value::String(value.to_string()));
                }
            }
        }
    }
    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_invalid() {
        let ini_content = r#"[section"#;
        let result = deserialize(ini_content.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_global_section() {
        let ini_content = r#"
key1 = "value1"
key2 = "value2"
"#;
        let parsed_map = deserialize(ini_content.to_string()).unwrap();

        assert_eq!(
            parsed_map,
            Map::from_iter(vec![
                ("key1".to_string(), Value::String("value1".to_string())),
                ("key2".to_string(), Value::String("value2".to_string()))
            ])
        );
    }

    #[test]
    fn test_deserialize() {
        let ini_content = r#"
[section]
key = "value"
"#;
        let parsed_map = deserialize(ini_content.to_string()).unwrap();
        assert_eq!(
            parsed_map,
            Map::from_iter(vec![(
                "section".to_string(),
                Value::Table(Map::from_iter(vec![(
                    "key".to_string(),
                    Value::String("value".to_string())
                )]))
            )])
        );
    }
}
