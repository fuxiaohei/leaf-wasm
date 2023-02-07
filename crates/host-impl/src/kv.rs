wasmtime::component::bindgen!({
    world:"kv",
    path: "../../wit/kv.wit",
    async: true,
});

use async_trait::async_trait;
use kv_imports::OpResult;

/// KvImpl is the implementation of the `kv` component.
pub struct KvImpl {
    pub user_uuid: String,
    pub namespace: String,
}

impl KvImpl {
    /// new creates a new `KvImpl` instance.
    pub fn new(user_uuid: String, namespace: String) -> Self {
        KvImpl {
            user_uuid,
            namespace,
        }
    }
    /// build key with uuid and namespace
    fn build_key(&self, key: &str) -> String {
        format!("{}:{}:{}", self.user_uuid, self.namespace, key)
    }
}

#[async_trait]
impl kv_imports::KvImports for KvImpl {
    /// set sets a key-value pair in the key-value store.
    async fn set(&mut self, key: String, value: String, ttl: u32) -> anyhow::Result<OpResult> {
        Ok(OpResult::Success)
    }

    /// get gets a value from the key-value store.
    async fn get(&mut self, key: String) -> anyhow::Result<Result<String, OpResult>> {
        Ok(Ok("value".to_string()))
    }

    /// list_values lists all key-value pairs in the key-value store.
    async fn list_values(&mut self) -> anyhow::Result<Result<Vec<(String, String)>, OpResult>> {
        Ok(Ok(vec![("key1".to_string(), "value1".to_string())]))
    }

    /// delete deletes a key-value pair from the key-value store.
    async fn delete(&mut self, key: String) -> anyhow::Result<OpResult> {
        Ok(OpResult::Success)
    }
}

/// export the `kv` component.
pub use kv_imports::add_to_linker;
