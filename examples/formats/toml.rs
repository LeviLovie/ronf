use ronf::prelude::{Config, File, FileFormat};

fn main() {
    let config = Config::builder()
        .add_file(File::new_str(
            "test_file",
            FileFormat::Toml,
            r#"[section]
key = "value""#,
        ))
        .build()
        .unwrap();
    println!(
        "\"key\": {}",
        config.get("section").unwrap().get("key").unwrap()
    );
}
