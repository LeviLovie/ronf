use ronf::prelude::{Config, File, FileFormat};

fn main() {
    let default_file = File::new_str("test_file", FileFormat::Json, "{\"key\": \"value\"}");
    let save = {
        let mut config = Config::builder().add(default_file.clone()).build().unwrap();
        println!("\"key\": {}", config.get("key").unwrap());
        config.set("key", "another value".into());
        println!("\"key\" after change: {}", config.get("key").unwrap());
        config.save(FileFormat::Json).unwrap()
    };

    let loaded_config = Config::builder()
        .add(default_file.clone())
        .load(save, FileFormat::Json)
        .unwrap()
        .build()
        .unwrap();
    println!("\"key\" after load: {}", loaded_config.get("key").unwrap());
}
