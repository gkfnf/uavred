use sqlez_macros::sql;

use crate::{
    query,
    sqlez::{domain::Domain, thread_safe_connection::ThreadSafeConnection},
};

pub struct KeyValueStore(crate::sqlez::thread_safe_connection::ThreadSafeConnection);

impl Domain for KeyValueStore {
    const NAME: &str = stringify!(KeyValueStore);

    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;
    )];
}

crate::static_connection!(KEY_VALUE_STORE, KeyValueStore, []);

pub trait Dismissable {
    const KEY: &'static str;

    fn dismissed() -> bool {
        KEY_VALUE_STORE
            .read_kvp(Self::KEY)
            .ok()
            .flatten()
            .is_some()
    }
}

impl KeyValueStore {
    query! {
        pub fn read_kvp(key: &str) -> Result<Option<String>> {
            SELECT value FROM kv_store WHERE key = (?)
        }
    }

    pub async fn write_kvp(&self, key: String, value: String) -> anyhow::Result<()> {
        log::debug!("Writing key-value pair for key {}", key);
        self.write_kvp_inner(key, value).await
    }

    query! {
        async fn write_kvp_inner(key: String, value: String) -> Result<()> {
            INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))
        }
    }

    query! {
        pub async fn delete_kvp(key: String) -> Result<()> {
            DELETE FROM kv_store WHERE key = (?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::kvp::KeyValueStore;

    #[test]
    fn test_kvp() {
        smol::block_on(async {
            let db = KeyValueStore::open_test_db("test_kvp").await;

            assert_eq!(db.read_kvp("key-1").unwrap(), None);

            db.write_kvp("key-1".to_string(), "one".to_string())
                .await
                .unwrap();
            assert_eq!(db.read_kvp("key-1").unwrap(), Some("one".to_string()));

            db.write_kvp("key-1".to_string(), "one-2".to_string())
                .await
                .unwrap();
            assert_eq!(db.read_kvp("key-1").unwrap(), Some("one-2".to_string()));

            db.write_kvp("key-2".to_string(), "two".to_string())
                .await
                .unwrap();
            assert_eq!(db.read_kvp("key-2").unwrap(), Some("two".to_string()));

            db.delete_kvp("key-1".to_string()).await.unwrap();
            assert_eq!(db.read_kvp("key-1").unwrap(), None);
        });
    }
}

pub struct GlobalKeyValueStore(ThreadSafeConnection);

impl Domain for GlobalKeyValueStore {
    const NAME: &str = stringify!(GlobalKeyValueStore);
    const MIGRATIONS: &[&str] = &[sql!(
        CREATE TABLE IF NOT EXISTS kv_store(
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) STRICT;
    )];
}

crate::static_connection!(GLOBAL_KEY_VALUE_STORE, GlobalKeyValueStore, [], global);

impl GlobalKeyValueStore {
    query! {
        pub fn read_kvp(key: &str) -> Result<Option<String>> {
            SELECT value FROM kv_store WHERE key = (?)
        }
    }

    pub async fn write_kvp(&self, key: String, value: String) -> anyhow::Result<()> {
        log::debug!("Writing global key-value pair for key {}", key);
        self.write_kvp_inner(key, value).await
    }

    query! {
        async fn write_kvp_inner(key: String, value: String) -> Result<()> {
            INSERT OR REPLACE INTO kv_store(key, value) VALUES ((?), (?))
        }
    }

    query! {
        pub async fn delete_kvp(key: String) -> Result<()> {
            DELETE FROM kv_store WHERE key = (?)
        }
    }
}
