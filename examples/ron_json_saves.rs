//! Save changes to RON default config in json format.
use ronf::prelude::{Config, File, FileFormat, Value};

fn main() {
    let defaults = File::new_str(
        "test_file",
        FileFormat::Ron,
        r#"(
    width: 800,
    height: 600,
    full_screen: false,
)"#,
    );

    let save = {
        let mut config = Config::builder()
            .add_file(defaults.clone())
            .build()
            .unwrap();
        println!("Defaults:\n{}", config);
        config.set("width", Value::Int(1000));
        config.set("height", Value::Int(800));
        println!("Before save:\n{}", config);
        config.save(FileFormat::Json).unwrap()
    };

    let loaded_config = Config::builder()
        .add_file(defaults)
        .load(File::new("save.json".to_string(), FileFormat::Json, save))
        .unwrap()
        .build()
        .unwrap();
    println!("After load:\n{}", loaded_config);
}
