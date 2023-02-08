/// KvError is the error type for the kv store.
#[derive(Debug)]
pub enum Error {
    /// OperationFailed is returned when the operation failed.
    OperationFailed,
    /// Expired is returned when the key has expired.
    Expired,
    /// NotExist is returned when the key was not found.
    NotExist,
    /// Ok is returned when the operation was successful.
    Ok,
}

/// Provider is the interface for a key-value store.
#[async_trait::async_trait]
pub trait Provider: Send {
    async fn get(&mut self, key: String) -> Result<String, Error>;
    async fn set(&mut self, key: String, value: String, ttl: u32) -> Result<(), Error>;
    async fn delete(&mut self, key: String) -> Result<(), Error>;
    async fn list(&mut self, prefix: String) -> Result<Vec<(String, String)>, Error>;
}

mod memory;
pub use memory::Memory;

mod dummy;
pub use dummy::Dummy;