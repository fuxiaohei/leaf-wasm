include!("../../../wit/kv.rs");

/// KvError is the error type returned by the kv functions.
type KvError = kv_imports::OpResult;

/// set sets a key-value pair in the kv store.
pub fn set(key: String, value: String, ttl: Option<u32>) -> Result<(), KvError> {
    let ttl = match ttl {
        Some(t) => t,
        None => 0,
    };
    let op = kv_imports::set(key.as_str(), value.as_str(), ttl);
    if op == KvError::Success {
        Ok(())
    } else {
        Err(op)
    }
}

/// get gets a value from the kv store.
pub fn get(key: String) -> Result<String, KvError> {
    let op = kv_imports::get(key.as_str());
    if op.is_err() {
        Err(op.err().unwrap())
    } else {
        Ok(op.unwrap())
    }
}

/// list lists all values for a given key.
pub fn list() -> Result<Vec<String>, KvError> {
    let op = kv_imports::list_values();
    if op.is_err() {
        Err(op.err().unwrap())
    } else {
        Ok(op.unwrap())
    }
}
