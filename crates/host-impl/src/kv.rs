wasmtime::component::bindgen!({
    world:"kv",
    path: "../../wit/kv.wit",
    async: true,
});

use async_trait::async_trait;
use kv_imports::OpResult;
use leaf_host_kv::{Error as KvError, Provider};

/// KvImpl is the implementation of the `kv` component.
pub struct KvImpl {
    pub user_uuid: String,
    pub namespace: String,
    inner: Box<dyn Provider>,
}

impl KvImpl {
    /// new creates a new `KvImpl` instance.
    pub fn new(user_uuid: String, namespace: String, inner: Box<dyn Provider>) -> Self {
        KvImpl {
            user_uuid,
            namespace,
            inner,
        }
    }
    /// build key with uuid and namespace
    fn build_key(&self, key: String) -> String {
        format!("{}/{}/{}", self.user_uuid, self.namespace, key)
    }
}

impl TryInto<OpResult> for KvError {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<OpResult, <Self as TryInto<OpResult>>::Error> {
        // Error ambiguous associated type
        match self {
            KvError::Ok => Ok(OpResult::Success),
            KvError::Expired => Ok(OpResult::Expired),
            KvError::NotExist => Ok(OpResult::NotExist),
            KvError::OperationFailed => Ok(OpResult::Error),
        }
    }
}

#[async_trait]
impl kv_imports::KvImports for KvImpl {
    /// set sets a key-value pair in the key-value store.
    async fn set(&mut self, key: String, value: String, ttl: u32) -> anyhow::Result<OpResult> {
        let key = self.build_key(key);
        let result = self.inner.set(key, value, ttl).await.unwrap_err();
        Ok(result.try_into().unwrap())
    }

    /// get gets a value from the key-value store.
    async fn get(&mut self, key: String) -> anyhow::Result<Result<String, OpResult>> {
        let key = self.build_key(key);
        let result = self.inner.get(key).await;
        if result.is_err() {
            let result = result.unwrap_err();
            Ok(Err(result.try_into().unwrap()))
        } else {
            Ok(Ok(result.unwrap()))
        }
    }

    /// list_values lists all key-value pairs in the key-value store.
    async fn list_values(&mut self) -> anyhow::Result<Result<Vec<(String, String)>, OpResult>> {
        let prefix = self.build_key("".to_string());
        let result = self.inner.list(prefix).await;
        if result.is_err() {
            let result = result.unwrap_err();
            Ok(Err(result.try_into().unwrap()))
        }
        // trim prefix
        else {
            let mut result = result.unwrap();
            for (key, _) in result.iter_mut() {
                *key = key
                    .trim_start_matches(&self.build_key("".to_string()))
                    .to_string();
            }
            Ok(Ok(result))
        }
    }

    /// delete deletes a key-value pair from the key-value store.
    async fn delete(&mut self, key: String) -> anyhow::Result<OpResult> {
        let key = self.build_key(key);
        let result = self.inner.delete(key).await.unwrap_err();
        Ok(result.try_into().unwrap())
    }
}

/// export the `kv` component.
pub use kv_imports::add_to_linker;
