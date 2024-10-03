
use std::collections::HashMap;

pub struct KVStore {
    store: HashMap<String, String>,
}

impl KVStore {
    pub fn new() -> KVStore {
        KVStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_store_new() {
        let kv_store = KVStore::new();
        assert!(kv_store.store.is_empty(), "Store should be empty on initialization");
    }

    #[test]
    fn test_kv_store_get_non_existent_key() {
        let mut kv_store = KVStore::new();
        assert_eq!(kv_store.get("nonexistent"), None, "Getting a nonexistent key should return None");
    }

    #[test]
    fn test_kv_store_delete() {
        let mut kv_store = KVStore::new();
        kv_store.set("key1".to_string(), "value1".to_string());
        assert_eq!(kv_store.delete("key1"), Some("value1".to_string()), "Delete should return the value");
        assert_eq!(kv_store.get("key1"), None, "Key should no longer exist after delete");
    }

    #[test]
    fn test_kv_store_delete_non_existent_key() {
        let mut kv_store = KVStore::new();
        assert_eq!(kv_store.delete("nonexistent"), None, "Deleting a non existing value should return None");
    }
}
