use crate::value::{Map, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileFormat {
    Ini,
    Json,
    Yaml,
    Toml,
}

impl FileFormat {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "ini" => Some(FileFormat::Ini),
            "json" => Some(FileFormat::Json),
            "yaml" => Some(FileFormat::Yaml),
            "toml" => Some(FileFormat::Toml),
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
        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

        let extension = path
            .split('.')
            .last()
            .ok_or_else(|| format!("Failed to get file extension from {}", path))?;
        let format = FileFormat::from_extension(extension)
            .ok_or_else(|| format!("Unsupported file extension: {}", extension))?;

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
                    unimplemented!("Parsing INI file: {}", self.path);
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
                    unimplemented!("Parsing TOML file: {}", self.path);
                }

                #[cfg(not(feature = "toml"))]
                Err("TOML format feature is not enabled".to_string())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_file() {
        let path = "test.json".to_string();
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#.to_string();
        let file = File::new(path.clone(), format.clone(), content.clone());
        assert_eq!(file.path, path);
        assert_eq!(file.format, format);
        assert_eq!(file.content, content);
    }

    #[test]
    fn test_parse_json() {
        let path = "test.json".to_string();
        let format = FileFormat::Json;
        let content = r#"{"key": "value"}"#.to_string();
        let file = File::new(path.clone(), format.clone(), content.clone());
        let result = file.parse();
        assert!(result.is_ok());
    }
}
