include!("../../../wit/kv.rs");

use kv_imports::OpResult;
use leaf_host_kv::Error as KvError;

/// TryInto is implemented for OpResult to convert it to KvError.
impl TryInto<KvError> for OpResult {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<KvError, <Self as TryInto<KvError>>::Error> {
        // Error ambiguous associated type
        match self {
            OpResult::Success => Ok(KvError::Ok),
            OpResult::Expired => Ok(KvError::Expired),
            OpResult::NotExist => Ok(KvError::NotExist),
            OpResult::Error => Ok(KvError::OperationFailed),
        }
    }
}

/// set sets a key-value pair in the kv store.
pub fn set(key: String, value: String, ttl: Option<u32>) -> Result<(), KvError> {
    let ttl = match ttl {
        Some(t) => t,
        None => 0,
    };
    let op = kv_imports::set(key.as_str(), value.as_str(), ttl);
    if op == kv_imports::OpResult::Success {
        Ok(())
    } else {
        Err(op.try_into().unwrap())
    }
}

/// get gets a value from the kv store.
pub fn get(key: String) -> Result<String, KvError> {
    let op = kv_imports::get(key.as_str());
    if op.is_err() {
        Err(op.err().unwrap().try_into().unwrap())
    } else {
        Ok(op.unwrap())
    }
}

/// list lists all values for a given key.
pub fn list() -> Result<Vec<(String, String)>, KvError> {
    let op = kv_imports::list_values();
    if op.is_err() {
        Err(op.err().unwrap().try_into().unwrap())
    } else {
        Ok(op.unwrap())
    }
}
