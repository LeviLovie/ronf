[![Rust](https://github.com/LeviLovie/ronf/actions/workflows/ci.yaml/badge.svg)](https://github.com/LeviLovie/ronf/actions)
[![Docs](https://docs.rs/ronf/badge.svg)](https://docs.rs/ronf)
[![Coverage](https://coveralls.io/repos/github/LeviLovie/ronf/badge.svg?branch=main)](https://coveralls.io/github/LeviLovie/ronf?branch=main)
[![Crates](https://img.shields.io/crates/v/ronf.svg)](https://crates.io/crates/ronf)
[![License](https://img.shields.io/crates/l/ronf.svg)](https://choosealicense.com/licenses/mit/)
# Ronf

A configuration library with saving based on [config-rs](https://github.com/rust-cli/config-rs/tree/main).

## Examples

```rust
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
```

For more examples, check `examples/`. Run with `cargo run --example FILE_NAME`.

## Features

- `ordered` - Uses HashMap from `indexmap` instead of `std::collections` to preserve order of arrays;
- `load_after_build` - Enables loading saves on `Config` (After building with `ConfigBuilder::build()`);
- `read_file` - Add functions to read `File` from path;
- `env` - Adds `.env()` on `ConfigBuilder` to overwrite keys with env vars.

### File formats

- `ini` - Load [Ini files](https://en.wikipedia.org/wiki/INI_file).
- `json` - Load [Json files](https://en.wikipedia.org/wiki/JSON).
- `yaml` - Load [Yaml files](https://en.wikipedia.org/wiki/YAML).
- `toml` - Load [Toml files](https://en.wikipedia.org/wiki/TOML).
