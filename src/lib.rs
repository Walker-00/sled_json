use std::path::Path;

use serde::{de::Deserialize as DeDeserialize, ser::Serialize as SerSerialize};
pub use serde::{Deserialize, Serialize};
use sled::{Db, IVec, Result};

pub struct JsonDb {
    db: Db,
}

impl JsonDb {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<JsonDb> {
        let db = sled::open(path)?;
        Ok(JsonDb { db })
    }

    pub fn insert<K: AsRef<[u8]>, V: ?Sized + SerSerialize>(
        &self,
        key: K,
        value: &V,
    ) -> Result<Option<IVec>> {
        let value = serde_json::to_string(value).unwrap();
        self.db.insert(key, value.as_bytes())
    }

    pub fn get<T: for<'a> DeDeserialize<'a>>(
        &self,
        key: &dyn AsRef<[u8]>,
    ) -> std::result::Result<Option<T>, sled::Error> {
        match self.db.get(key.as_ref())? {
            Some(v) => {
                let x = String::from_utf8(v.to_vec()).unwrap();
                match serde_json::from_str::<T>(&x) {
                    Ok(x_v) => Ok(Some(x_v)),
                    Err(_) => Err(sled::Error::ReportableBug(
                        "Error in Json parsing to given data structure!!".into(),
                    )),
                }
            }
            None => Ok(None),
        }
    }
}

#[test]

fn db_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Info {
        name: String,
        age: u8,
    }

    let linus_walker = Info {
        name: "Linus Walker".into(),
        age: 14,
    };
    let db = JsonDb::open("db").unwrap();
    db.insert("linus_walker", &linus_walker).unwrap();
    let resul = db.get::<Info>(&"linus_walker".to_string()).unwrap();
    println!("{resul:?}");
}
