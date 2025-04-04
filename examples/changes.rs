use ronf::prelude::{Config, File, FileFormat};

fn main() {
    let mut config = Config::builder()
        .add(File::new_str(
            "test_file",
            FileFormat::Json,
            "{\"key\": \"value\"}",
        ))
        .build()
        .unwrap();
    println!("\"key\": {}", config.get("key").unwrap());
    config.set("key", "another value".into());
    println!("\"key\" after change: {}", config.get("key").unwrap());
}
