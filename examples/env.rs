use ronf::{Config, File, FileFormat};

fn main() {
    unsafe {
        std::env::set_var("KEY", "overwrite");
    }

    let config = Config::builder()
        .add_file(File::new_str(
            "test_file",
            FileFormat::Json,
            "{\"key\": \"value\"}",
        ))
        .build()
        .unwrap();
    println!("\"key\": {}", config.get("key").unwrap());
}
