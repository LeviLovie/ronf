use ronf::prelude::{Config, File, FileFormat};

fn main() {
    let config = Config::builder()
        .add_file(File::new_str(
            "test_file",
            FileFormat::Ron,
            r#"
            (
                key: "value",
            )"#,
        ))
        .build()
        .unwrap();
    println!("\"key\": {}", config.get("key").unwrap());
}
