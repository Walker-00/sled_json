<div align=center>
    
[![Rust](https://github.com/Walker-00/sled_json/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/sled_json/actions/workflows/rust.yml)

</div>

# sled_json
Rust based Sled Key-Value store db as Key-Structured Value Warpper

# Usage

<h3> First We need to add require dependencies in <code> Cargo.toml </code> </h3>

```rs
use sled_json::{JsonDb, Serialize, Deserialize};

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
Just a Json Based Warpper for [SledDb](https://github.com/spacejam/sled)
