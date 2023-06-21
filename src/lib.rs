use std::path::Path;

use serde::{de::Deserialize, ser::Serialize};
use sled::{Db, IVec, Result};

pub struct JsonDb {
    db: Db,
}

impl JsonDb {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<JsonDb> {
        let db = sled::open(path)?;
        Ok(JsonDb { db })
    }

    pub fn insert<K: AsRef<[u8]>, V: ?Sized + Serialize>(
        &self,
        key: K,
        value: &V,
    ) -> Result<Option<IVec>> {
        let value = serde_json::to_string(value).unwrap();
        Ok(self.db.insert(key, value.as_bytes())?)
    }

    pub fn get<T: for<'a> Deserialize<'a>>(
        &self,
        key: &dyn AsRef<[u8]>,
    ) -> std::result::Result<T, serde_json::Error> {
        let v = self.db.get(key.as_ref()).unwrap().unwrap();
        let x = String::from_utf8(v.to_vec()).unwrap();
        serde_json::from_str(&x)
    }
}
