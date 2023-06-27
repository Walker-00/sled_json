<div align=center>
    
[![Rust](https://github.com/Walker-00/sled_json/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/sled_json/actions/workflows/rust.yml)

</div>

# sled_json
Rust based Sled Key-Value store db as Key-Structured Value Wrapper

# Usage

<h2> Cargo.toml </h2>

```toml
[dependencies]
serde = { version = "1.0.164", features = ["derive"] }
sled = { git = "https://github.com/Walker-00/sled_json" }
```
<h2>main.rs</h2>

```rs
use serde::{Serialize, Deserialize};
use sled_json::JsonDb;

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    name: String,
    age: u8,
}

fn main() {
    let linus_walker = Info {
        name: "Linus Walker".into(),
        age: 14,
    };
    let db = JsonDb::open("db").unwrap();
    db.insert("linus_walker", &linus_walker).unwrap();
    let resul = db.get::<Info>(&"linus_walker".to_string()).unwrap();
    println!("{resul:?}");
}
```

# Credit
Just a Json Based Wrapper for [SledDb](https://github.com/spacejam/sled) with the help of [Serde](https://github.com/serde-rs/serde)
