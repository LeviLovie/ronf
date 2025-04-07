//! Configuration structure

use crate::file::{File, FileFormat};
use crate::value::{Map, Value};

/// Builder for the Config struct
pub struct ConfigBuilder {
    pub files: Vec<File>,
    pub changes: Map<String, Value>,
}

impl ConfigBuilder {
    /// Creates a new ConfigBuilder instance
    pub fn build(self) -> Result<Config, String> {
        let mut config = Config {
            defaults: Map::new(),
            changes: Map::new(),
            values: Map::new(),
        };

        for file in self.files {
            let parsed = file
                .parse()
                .map_err(|e| format!("Failed to parse file {}: {}", file.path, e))?;
            config.defaults.extend(parsed);
        }

        config.values = config.defaults.clone();

        for (key, value) in self.changes.iter() {
            if config.values.contains_key(key) {
                config.values.insert(key.clone(), value.clone());
            }
        }

        #[cfg(feature = "env")]
        {
            let env_vars = get_env_vars();
            for (key, value) in env_vars.iter() {
                let key = key.to_lowercase();
                let mut key_parts: Vec<&str> = key.split('_').collect();
                key_parts.retain(|&part| !part.is_empty());
                if key_parts.is_empty() {
                    continue;
                }

                let val = match config.values.get(key_parts[0]) {
                    Some(v) => v,
                    None => {
                        continue;
                    }
                };
                if !val.is_table() {
                    *config.values.get_mut(key_parts[0]).unwrap() = value.clone();
                    continue;
                }
            }
        }

        Ok(config)
    }

    /// Adds a file to the builder
    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    /// Loads changes to default configuration from `.add_file()` from a file.
    /// Example:
    /// ```rust
    /// use ronf::{Config, File, FileFormat};
    /// let default_file = File::new_str("test_file", FileFormat::Json, "{\"key\": \"value\"}");
    /// let save = {
    ///     let mut config = Config::builder()
    ///         .add_file(default_file.clone())
    ///         .build()
    ///         .unwrap();
    ///     println!("\"key\": {}", config.get("key").unwrap());
    ///     config.set("key", "another value".into());
    ///     println!("\"key\" after change: {}", config.get("key").unwrap());
    ///     config.save(FileFormat::Json).unwrap()
    /// };
    /// let loaded_config = Config::builder()
    ///     .add_file(default_file.clone())
    ///     .load(File::new("save.json".to_string(), FileFormat::Json, save))
    ///     .unwrap()
    ///     .build()
    ///     .unwrap();
    /// println!("\"key\" after load: {}", loaded_config.get("key").unwrap());
    /// ```
    pub fn load(mut self, file: File) -> Result<Self, String> {
        self.changes = load_map(file.content, file.format)?;
        Ok(self)
    }
}

#[cfg(feature = "env")]
fn get_env_vars() -> Map<String, Value> {
    let mut env_vars = Map::new();
    for (key, value) in std::env::vars() {
        env_vars.insert(key, Value::String(value));
    }
    env_vars
}

/// Configuration structure to hold parsed values
///
/// Simple example:
/// ```rust
/// #[cfg(features = "json")]
/// {
/// use ronf::{Config, File, FileFormat};
/// let config = Config::builder().add_file(File::new_str("test_file", FileFormat::Json, "{\"key\":
/// \"value\"}")).build().unwrap();
/// println!("\"key\": {}", config.get("key").unwrap());
/// }
/// ```
pub struct Config {
    defaults: Map<String, Value>,
    changes: Map<String, Value>,
    values: Map<String, Value>,
}

impl Config {
    /// Creates a ConfigBuilder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            files: Vec::new(),
            changes: Map::new(),
        }
    }

    /// Get a value from config using a key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    /// Set a value in config changes using a key
    pub fn set(&mut self, key: &str, value: Value) {
        self.changes.insert(key.to_string(), value.clone());
        self.values.insert(key.to_string(), value);
    }

    /// List all keys in the config
    pub fn list(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    /// Load changes to default configuration from `.add_file()` from a file.
    #[cfg(feature = "load_after_build")]
    pub fn load(&mut self, file: File) -> Result<(), String> {
        let parsed = file
            .parse()
            .map_err(|e| format!("Failed to parse file {}: {}", file.path, e))?;
        self.changes.extend(parsed);
        self.values = self.defaults.clone();
        for (key, value) in self.changes.iter() {
            if self.values.get(key).is_some() {
                self.values.insert(key.clone(), value.clone());
            }
        }
        Ok(())
    }

    /// Save the current configuration to a file in the specified format
    pub fn save(&self, format: FileFormat) -> Result<String, String> {
        save_map(&self.changes, format)
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, val) in self.values.iter() {
            writeln!(f, "{}: {}", key, val)?;
        }
        Ok(())
    }
}

fn save_map(_map: &Map<String, Value>, format: FileFormat) -> Result<String, String> {
    match format {
        FileFormat::Ini => {
            #[cfg(feature = "ini")]
            {
                Err("Serializing INI format is not supported".to_string())
            }

            #[cfg(not(feature = "ini"))]
            Err("INI format feature is not enabled".to_string())
        }
        FileFormat::Json => {
            #[cfg(feature = "json")]
            {
                Ok(crate::format::json::serialize(_map.clone()))
            }

            #[cfg(not(feature = "json"))]
            Err("JSON format feature is not enabled".to_string())
        }
        FileFormat::Yaml => {
            #[cfg(feature = "yaml")]
            {
                Ok(crate::format::yaml::serialize(_map.clone()))
            }

            #[cfg(not(feature = "yaml"))]
            Err("YAML format feature is not enabled".to_string())
        }
        FileFormat::Toml => {
            #[cfg(feature = "toml")]
            {
                Ok(crate::format::toml::serialize(_map.clone()))
            }

            #[cfg(not(feature = "toml"))]
            Err("TOML format feature is not enabled".to_string())
        }
        FileFormat::Ron => {
            #[cfg(feature = "ron")]
            {
                Ok(crate::format::ron::serialize(_map.clone()))
            }

            #[cfg(not(feature = "ron"))]
            Err("RON format feature is not enabled".to_string())
        }
    }
}

fn load_map(save: String, format: FileFormat) -> Result<Map<String, Value>, String> {
    if save.is_empty() {
        return Err("Empty content".to_string());
    }

    match format {
        FileFormat::Ini => {
            #[cfg(feature = "ini")]
            {
                crate::format::ini::deserialize(save.clone())
            }

            #[cfg(not(feature = "ini"))]
            Err("INI format feature is not enabled".to_string())
        }
        FileFormat::Json => {
            #[cfg(feature = "json")]
            {
                crate::format::json::deserialize(save.clone())
            }

            #[cfg(not(feature = "json"))]
            Err("JSON format feature is not enabled".to_string())
        }
        FileFormat::Yaml => {
            #[cfg(feature = "yaml")]
            {
                crate::format::yaml::deserialize(save.clone())
            }

            #[cfg(not(feature = "yaml"))]
            Err("YAML format feature is not enabled".to_string())
        }
        FileFormat::Toml => {
            #[cfg(feature = "toml")]
            {
                crate::format::toml::deserialize(save.clone())
            }

            #[cfg(not(feature = "toml"))]
            Err("TOML format feature is not enabled".to_string())
        }
        FileFormat::Ron => {
            #[cfg(feature = "ron")]
            {
                crate::format::ron::deserialize(save.clone())
            }

            #[cfg(not(feature = "ron"))]
            Err("RON format feature is not enabled".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config_builder() {
        let _config = Config::builder();
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_config_get() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key1\": \"value\"}",
            ))
            .build()
            .unwrap();
        assert_eq!(
            config.get("key1").unwrap(),
            &Value::String("value".to_string())
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_config_set() {
        let mut config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key2\": \"value\"}",
            ))
            .build()
            .unwrap();
        config.set("key2", Value::String("new_value".to_string()));
        assert_eq!(
            config.get("key2").unwrap(),
            &Value::String("new_value".to_string())
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_config_list() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key3\": \"value\"}",
            ))
            .build()
            .unwrap();
        assert_eq!(config.list(), vec!["key3".to_string()]);
    }

    #[cfg(feature = "json")]
    mod config_display {
        use super::*;
        use std::fmt::{self, Write};

        #[test]
        fn test_config_display() {
            let config = Config::builder()
                .add_file(File::new_str(
                    "test_file",
                    FileFormat::Json,
                    "{\"key3_2\": \"value\"}",
                ))
                .build()
                .unwrap();

            let mut output = String::new();
            let result = write!(&mut output, "{}", config);

            assert!(result.is_ok());
            assert_eq!(output, "key3_2: \"value\"\n");
        }

        struct FailingWriter;

        impl Write for FailingWriter {
            fn write_str(&mut self, _s: &str) -> fmt::Result {
                Err(fmt::Error) // Simulate a write failure
            }
        }

        #[test]
        fn test_config_display_write_error() {
            let config = Config::builder()
                .add_file(File::new_str(
                    "test_file",
                    FileFormat::Json,
                    "{\"key3_2\": \"value\"}",
                ))
                .build()
                .unwrap();

            let mut failing_writer = FailingWriter;
            let result = write!(&mut failing_writer, "{}", config);

            assert!(result.is_err()); // Ensure that write errors propagate
        }
    }

    #[test]
    #[cfg(feature = "load_after_build")]
    #[cfg(feature = "json")]
    fn test_config_load() {
        let mut config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key4\": \"value\"}",
            ))
            .build()
            .unwrap();
        config
            .load(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key4\": \"new_value\", \"key5\": \"another_value\"}",
            ))
            .unwrap();
        assert_eq!(
            config.get("key4").unwrap(),
            &Value::String("new_value".to_string())
        );

        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key6\": \"value\"}",
            ))
            .build()
            .unwrap()
            .load(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key6\": \"new_value}",
            ));
        assert!(config.is_err());
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_config_save() {
        let mut config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key7\": \"value\"}",
            ))
            .build()
            .unwrap();
        config.set("key7", Value::String("new_value".to_string()));
        let save = config.save(FileFormat::Json).unwrap();
        assert_eq!(save, "{\"key7\":\"new_value\"}");
    }

    #[test]
    fn test_builder_failed_parse_file() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key8\": \"value}",
            ))
            .build();
        assert!(config.is_err());
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_builder_load() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key9\": \"value\"}",
            ))
            .load(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key9\": \"new_value\"}",
            ))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            config.get("key9").unwrap(),
            &Value::String("new_value".to_string())
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_builder_load_failure() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key10\": \"value\"}",
            ))
            .load(File::new_str("test_file", FileFormat::Json, ""));
        assert!(config.is_err());
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_builder_load_none() {
        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key11\": \"value\"}",
            ))
            .load(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key12\": \"new_value\"}",
            ))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            config.get("key11").unwrap(),
            &Value::String("value".to_string())
        );
        assert!(config.get("key12").is_none());
    }

    #[test]
    #[cfg(feature = "env")]
    fn test_env_vars() {
        unsafe {
            std::env::set_var("KEY13", "overwrite");
        }

        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key13\": \"value\"}",
            ))
            .build()
            .unwrap();
        assert_eq!(
            config.get("key13").unwrap(),
            &Value::String("overwrite".to_string())
        );

        unsafe {
            std::env::remove_var("KEY13");
        }
    }

    #[test]
    #[cfg(feature = "env")]
    fn test_env_vars_table() {
        unsafe {
            std::env::set_var("KEY14", "overwrite");
        }

        let config = Config::builder()
            .add_file(File::new_str(
                "test_file",
                FileFormat::Json,
                "{\"key14\": {\"key15\": \"value\"}}",
            ))
            .build()
            .unwrap();
        let mut expected = Map::new();
        expected.insert("key15".to_string(), Value::String("value".to_string()));
        assert_eq!(config.get("key14").unwrap(), &Value::Table(expected));

        unsafe {
            std::env::remove_var("KEY14");
        }
    }

    mod serialize_deserialize {
        use super::*;

        #[test]
        #[cfg(feature = "ini")]
        fn test_deserialize_ini() {
            let ini = r#"[section]
key: "value""#;
            let map = load_map(ini.to_string(), FileFormat::Ini);
            assert!(map.is_ok());
        }

        #[test]
        #[cfg(feature = "ini")]
        fn test_serialize_ini() {
            let map = Map::new();
            let ini = save_map(&map, FileFormat::Ini);
            assert!(ini.is_err());
        }

        #[test]
        #[cfg(not(feature = "ini"))]
        fn test_deserialize_init_failure() {
            let ini = r#"[section]
key: "value""#;
            let map = load_map(ini.to_string(), FileFormat::Ini);
            assert!(map.is_err());
        }

        #[test]
        #[cfg(not(feature = "ini"))]
        fn test_serialize_ini_failure() {
            let map = Map::new();
            let ini = save_map(&map, FileFormat::Ini);
            assert!(ini.is_err());
        }

        #[test]
        #[cfg(feature = "json")]
        fn test_deserialize_json() {
            let json = r#"{"key": "value"}"#;
            let map = load_map(json.to_string(), FileFormat::Json);
            assert!(map.is_ok());
        }

        #[test]
        #[cfg(feature = "json")]
        fn test_serialize_json() {
            let map = Map::new();
            let json = save_map(&map, FileFormat::Json).unwrap();
            assert_eq!(json, "{}");
        }

        #[test]
        #[cfg(not(feature = "json"))]
        fn test_deserialize_json_failure() {
            let json = r#"{"key": "value"}"#;
            let map = load_map(json.to_string(), FileFormat::Json);
            assert!(map.is_err());
        }

        #[test]
        #[cfg(not(feature = "json"))]
        fn test_serialize_json_failure() {
            let map = Map::new();
            let json = save_map(&map, FileFormat::Json);
            assert!(json.is_err());
        }

        #[test]
        #[cfg(feature = "yaml")]
        fn test_deserialize_yaml() {
            let yaml = r#"key: value"#;
            let map = load_map(yaml.to_string(), FileFormat::Yaml);
            assert!(map.is_ok());
        }

        #[test]
        #[cfg(feature = "yaml")]
        fn test_serialize_yaml() {
            let map = Map::new();
            let yaml = save_map(&map, FileFormat::Yaml).unwrap();
            assert_eq!(yaml, "---\n{}");
        }

        #[test]
        #[cfg(not(feature = "yaml"))]
        fn test_deserialize_yaml_failure() {
            let yaml = r#"key: value"#;
            let map = load_map(yaml.to_string(), FileFormat::Yaml);
            assert!(map.is_err());
        }

        #[test]
        #[cfg(not(feature = "yaml"))]
        fn test_serialize_yaml_failure() {
            let map = Map::new();
            let yaml = save_map(&map, FileFormat::Yaml);
            assert!(yaml.is_err());
        }

        #[test]
        #[cfg(feature = "toml")]
        fn test_deserialize_toml() {
            let toml = r#"
val = "value""#;
            let map = load_map(toml.to_string(), FileFormat::Toml);
            assert!(map.is_ok());
        }

        #[test]
        #[cfg(feature = "toml")]
        fn test_serialize_toml() {
            let map = Map::new();
            let toml = save_map(&map, FileFormat::Toml).unwrap();
            assert_eq!(toml, "");
        }

        #[test]
        #[cfg(not(feature = "toml"))]
        fn test_deserialize_toml_failure() {
            let toml = r#"
key = "value""#;
            let map = load_map(toml.to_string(), FileFormat::Toml);
            assert!(map.is_err());
        }

        #[test]
        #[cfg(not(feature = "toml"))]
        fn test_serialize_toml_failure() {
            let map = Map::new();
            let toml = save_map(&map, FileFormat::Toml);
            assert!(toml.is_err());
        }

        #[test]
        #[cfg(feature = "ron")]
        fn test_deserialize_ron() {
            let ron = r#"(key: "value")"#;
            let map = load_map(ron.to_string(), FileFormat::Ron);
            assert!(map.is_ok());
        }

        #[test]
        #[cfg(feature = "ron")]
        fn test_serialize_ron() {
            let map = Map::new();
            let ron = save_map(&map, FileFormat::Ron).unwrap();
            assert_eq!(ron, "{}");
        }

        #[test]
        #[cfg(not(feature = "ron"))]
        fn test_deserialize_ron_failure() {
            let ron = r#"(key: "value")"#;
            let map = load_map(ron.to_string(), FileFormat::Ron);
            assert!(map.is_err());
        }

        #[test]
        #[cfg(not(feature = "ron"))]
        fn test_serialize_ron_failure() {
            let map = Map::new();
            let ron = save_map(&map, FileFormat::Ron);
            assert!(ron.is_err());
        }
    }
}
