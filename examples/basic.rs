use ronf::prelude::{Config, File, FileFormat};

fn main() {
    let config = Config::builder()
        .add(File::new_str(
            "test_file",
            FileFormat::Json,
            "{\"key\": \"value\"}",
        ))
        .build()
        .unwrap();
    println!("\"key\": {}", config.get("key").unwrap());
}
