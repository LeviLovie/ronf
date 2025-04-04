use crate::file::{File, FileFormat};
use crate::value::{Map, Value};

pub struct ConfigBuilder {
    pub files: Vec<File>,
    pub changes: Map<String, Value>,
}

impl ConfigBuilder {
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
            if config.values.get(key).is_some() {
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

    pub fn add_file(mut self, file: File) -> Self {
        self.files.push(file);
        self
    }

    pub fn load(mut self, save: String, format: FileFormat) -> Result<Self, String> {
        self.changes = load_map(save, format)?;
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
/// use ronf::prelude::{Config, File, FileFormat};
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
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder {
            files: Vec::new(),
            changes: Map::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.changes.insert(key.to_string(), value.clone());
        self.values.insert(key.to_string(), value);
    }

    pub fn list(&self) -> Vec<String> {
        self.values.iter().map(|(key, _)| key.clone()).collect()
    }

    pub fn save(&self, format: FileFormat) -> Result<String, String> {
        save_map(&self.changes, format)
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

fn load_map(_save: String, format: FileFormat) -> Result<Map<String, Value>, String> {
    match format {
        FileFormat::Ini => {
            #[cfg(feature = "ini")]
            {
                crate::format::ini::deserialize(_save.clone())
            }

            #[cfg(not(feature = "ini"))]
            Err("INI format feature is not enabled".to_string())
        }
        FileFormat::Json => {
            #[cfg(feature = "json")]
            {
                crate::format::json::deserialize(_save.clone())
            }

            #[cfg(not(feature = "json"))]
            Err("JSON format feature is not enabled".to_string())
        }
        FileFormat::Yaml => {
            #[cfg(feature = "yaml")]
            {
                crate::format::yaml::deserialize(_save.clone())
            }

            #[cfg(not(feature = "yaml"))]
            Err("YAML format feature is not enabled".to_string())
        }
        FileFormat::Toml => {
            #[cfg(feature = "toml")]
            {
                crate::format::toml::deserialize(_save.clone())
            }

            #[cfg(not(feature = "toml"))]
            Err("TOML format feature is not enabled".to_string())
        }
        FileFormat::Ron => {
            #[cfg(feature = "ron")]
            {
                crate::format::ron::deserialize(_save.clone())
            }

            #[cfg(not(feature = "ron"))]
            Err("RON format feature is not enabled".to_string())
        }
    }
}
