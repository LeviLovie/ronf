use crate::value::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFormat {
    Ini,
    Json,
    Yaml,
    Toml,
    Ron,
}

impl FileFormat {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "ini" => Some(FileFormat::Ini),
            "json" => Some(FileFormat::Json),
            "yaml" => Some(FileFormat::Yaml),
            "toml" => Some(FileFormat::Toml),
            "ron" => Some(FileFormat::Ron),
            _ => None,
        }
    }
}

impl std::fmt::Display for FileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileFormat::Ini => write!(f, "ini"),
            FileFormat::Json => write!(f, "json"),
            FileFormat::Yaml => write!(f, "yaml"),
            FileFormat::Toml => write!(f, "toml"),
            FileFormat::Ron => write!(f, "ron"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub path: String,
    pub format: FileFormat,
    pub content: String,
}

impl File {
    pub fn new(path: String, format: FileFormat, content: String) -> Self {
        File {
            path,
            format,
            content,
        }
    }

    pub fn new_str(path: &str, format: FileFormat, content: &str) -> Self {
        File {
            path: path.to_string(),
            format,
            content: content.to_string(),
        }
    }

    #[cfg(feature = "read_file")]
    pub fn from_path(path: String) -> Result<Self, String> {
        let extension = path
            .rsplit_once('.')
            .and_then(|(_, ext)| if ext.is_empty() { None } else { Some(ext) })
            .ok_or_else(|| format!("Failed to get file extension from {}", path))?;
        let format = FileFormat::from_extension(extension)
            .ok_or_else(|| format!("Unsupported file extension: {}", extension))?;

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

        Ok(File::new(path.clone(), format, content))
    }

    #[cfg(feature = "read_file")]
    pub fn from_path_format(path: String, format: FileFormat) -> Result<Self, String> {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

        Ok(File::new(path.clone(), format, content))
    }

    pub fn parse(&self) -> Result<Map<String, Value>, String> {
        match self.format {
            FileFormat::Ini => {
                #[cfg(feature = "ini")]
                {
                    crate::format::ini::deserialize(self.content.clone())
                }

                #[cfg(not(feature = "ini"))]
                Err("INI format feature is not enabled".to_string())
            }
            FileFormat::Json => {
                #[cfg(feature = "json")]
                {
                    crate::format::json::deserialize(self.content.clone())
                }

                #[cfg(not(feature = "json"))]
                Err("JSON format feature is not enabled".to_string())
            }
            FileFormat::Yaml => {
                #[cfg(feature = "yaml")]
                {
                    crate::format::yaml::deserialize(self.content.clone())
                }

                #[cfg(not(feature = "yaml"))]
                Err("YAML format feature is not enabled".to_string())
            }
            FileFormat::Toml => {
                #[cfg(feature = "toml")]
                {
                    crate::format::toml::deserialize(self.content.clone())
                }

                #[cfg(not(feature = "toml"))]
                Err("TOML format feature is not enabled".to_string())
            }
            FileFormat::Ron => {
                #[cfg(feature = "ron")]
                {
                    crate::format::ron::deserialize(self.content.clone())
                }

                #[cfg(not(feature = "ron"))]
                Err("RON format feature is not enabled".to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_new() {
        let path = "test.json".to_string();
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#.to_string();
        let file = File::new(path.clone(), format.clone(), content.clone());
        assert_eq!(file.path, path);
        assert_eq!(file.format, format);
        assert_eq!(file.content, content);
    }

    #[test]
    fn test_file_new_str() {
        let path = "test.json";
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#;
        let file = File::new_str(path, format.clone(), content);
        assert_eq!(file.path, path);
        assert_eq!(file.format, format);
        assert_eq!(file.content, content);
    }

    #[test]
    fn test_file_format_from_extension() {
        assert_eq!(FileFormat::from_extension("ini"), Some(FileFormat::Ini));
        assert_eq!(FileFormat::from_extension("json"), Some(FileFormat::Json));
        assert_eq!(FileFormat::from_extension("yaml"), Some(FileFormat::Yaml));
        assert_eq!(FileFormat::from_extension("toml"), Some(FileFormat::Toml));
        assert_eq!(FileFormat::from_extension("ron"), Some(FileFormat::Ron));
        assert_eq!(FileFormat::from_extension("txt"), None);
    }

    #[test]
    fn test_file_format_display() {
        assert_eq!(format!("{}", FileFormat::Ini), "ini");
        assert_eq!(format!("{}", FileFormat::Json), "json");
        assert_eq!(format!("{}", FileFormat::Yaml), "yaml");
        assert_eq!(format!("{}", FileFormat::Toml), "toml");
        assert_eq!(format!("{}", FileFormat::Ron), "ron");
    }

    #[test]
    #[cfg(feature = "read_file")]
    fn test_file_from_path() {
        let path = "test.json".to_string();
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#.to_string();
        std::fs::write(&path, &content).unwrap();
        let file = File::from_path(path.clone()).unwrap();
        assert_eq!(file.path, path);
        assert_eq!(file.format, format);
        assert_eq!(file.content, content);
        std::fs::remove_file(path.clone()).unwrap();

        let file = File::from_path("test.json".to_string());
        assert!(file.is_err());

        let file = File::from_path("test_yaml.".to_string());
        assert!(file.is_err());

        let file = File::from_path("test.txt".to_string());
        assert!(file.is_err());
    }

    #[test]
    #[cfg(feature = "read_file")]
    fn test_file_from_path_format() {
        let path = "test2.json".to_string();
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#.to_string();
        std::fs::write(&path, &content).unwrap();
        let file = File::from_path_format(path.clone(), format.clone()).unwrap();
        assert_eq!(file.path, path);
        assert_eq!(file.format, format);
        assert_eq!(file.content, content);
        std::fs::remove_file(path.clone()).unwrap();

        let file = File::from_path_format(path.clone(), FileFormat::Yaml);
        assert!(file.is_err());
    }

    mod formats {
        use super::*;

        #[test]
        #[cfg(feature = "ini")]
        fn test_parse_ini() {
            let path = "test.ini".to_string();
            let format = FileFormat::Ini;
            let content = r#"[section]
key: value"#
                .to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_ok());
        }

        #[test]
        #[cfg(not(feature = "ini"))]
        fn test_parse_ini_fail() {
            let path = "test.ini".to_string();
            let format = FileFormat::Ini;
            let content = r#"[section]
key: value"#
                .to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_err());
        }

        #[test]
        #[cfg(feature = "json")]
        fn test_parse_json() {
            let path = "test.json".to_string();
            let format = FileFormat::Json;
            let content = r#"{"key": "value"}"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_ok());
        }

        #[test]
        #[cfg(not(feature = "json"))]
        fn test_parse_json_fail() {
            let path = "test.json".to_string();
            let format = FileFormat::Json;
            let content = r#"{"key": "value"}"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_err());
        }

        #[test]
        #[cfg(feature = "yaml")]
        fn test_parse_yaml() {
            let path = "test.yaml".to_string();
            let format = FileFormat::Yaml;
            let content = r#"key: value"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_ok());
        }

        #[test]
        #[cfg(not(feature = "yaml"))]
        fn test_parse_yaml_fail() {
            let path = "test.yaml".to_string();
            let format = FileFormat::Yaml;
            let content = r#"key: value"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_err());
        }

        #[test]
        #[cfg(feature = "toml")]
        fn test_parse_toml() {
            let path = "test.toml".to_string();
            let format = FileFormat::Toml;
            let content = r#"key = "value""#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_ok());
        }

        #[test]
        #[cfg(not(feature = "toml"))]
        fn test_parse_toml_fail() {
            let path = "test.toml".to_string();
            let format = FileFormat::Toml;
            let content = r#"key = "value""#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_err());
        }

        #[test]
        #[cfg(feature = "ron")]
        fn test_parse_ron() {
            let path = "test.ron".to_string();
            let format = FileFormat::Ron;
            let content = r#"(key: "value")"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_ok());
        }

        #[test]
        #[cfg(not(feature = "ron"))]
        fn test_parse_ron_fail() {
            let path = "test.ron".to_string();
            let format = FileFormat::Ron;
            let content = r#"(key: "value")"#.to_string();
            let file = File::new(path.clone(), format.clone(), content.clone());
            let result = file.parse();
            assert!(result.is_err());
        }
    }
}
