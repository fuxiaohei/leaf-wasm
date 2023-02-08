use super::{Error, Provider};
use std::collections::HashMap;
use time::OffsetDateTime;

/// Memory kv provider
pub struct Memory {
    data: HashMap<String, (String, u32)>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl Provider for Memory {
    async fn get(&mut self, key: String) -> Result<String, Error> {
        if !self.data.contains_key(key.as_str()) {
            return Err(Error::NotExist);
        }
        let (value, ttl) = self.data.get(key.as_str()).unwrap();
        if ttl > &0 && *ttl < OffsetDateTime::now_utc().unix_timestamp() as u32 {
            self.data.remove(key.as_str());
            return Err(Error::Expired);
        }
        Ok(value.clone())
    }

    async fn set(&mut self, key: String, value: String, ttl: u32) -> Result<(), Error> {
        let ttl = if ttl > 0 {
            OffsetDateTime::now_utc().unix_timestamp() as u32 + ttl
        } else {
            0
        };
        self.data.insert(key, (value, ttl));
        Ok(())
    }

    async fn delete(&mut self, key: String) -> Result<(), Error> {
        if self.data.contains_key(key.as_str()) {
            self.data.remove(key.as_str());
        }
        Ok(())
    }

    async fn list(&mut self, prefix: String) -> Result<Vec<(String, String)>, Error> {
        let mut result = vec![];
        for (key, (value, ttl)) in self.data.iter() {
            if key.starts_with(prefix.as_str()) {
                if *ttl > 0 && *ttl < OffsetDateTime::now_utc().unix_timestamp() as u32 {
                    continue;
                }
                result.push((key.clone(), value.clone()));
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_get_set() {
        let mut memory = Memory::new();
        memory
            .set("key".to_string(), "value".to_string(), 0)
            .await
            .unwrap();
        assert_eq!(memory.get("key".to_string()).await.unwrap(), "value");
        memory.delete("key".to_string()).await.unwrap();
        assert_eq!(memory.get("key".to_string()).await.is_err(), true);
    }

    // test memory set expired
    #[tokio::test]
    async fn test_memory_set_expired() {
        let mut memory = Memory::new();
        memory
            .set("key".to_string(), "value".to_string(), 1)
            .await
            .unwrap();
        assert_eq!(memory.get("key".to_string()).await.unwrap(), "value");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        assert_eq!(memory.get("key".to_string()).await.is_err(), true);
    }

    // test memory list
    #[tokio::test]
    async fn test_memory_list() {
        let mut memory = Memory::new();
        memory
            .set("key1".to_string(), "value1".to_string(), 0)
            .await
            .unwrap();
        memory
            .set("key2".to_string(), "value2".to_string(), 0)
            .await
            .unwrap();
        let values = memory.list("key".to_string()).await.unwrap();
        assert_eq!(values.len(), 2);
        if values[0].0 == "key1" {
            assert_eq!(values[0].1, "value1");
            assert_eq!(values[1].0, "key2");
            assert_eq!(values[1].1, "value2");
        } else {
            assert_eq!(values[0].0, "key2");
            assert_eq!(values[0].1, "value2");
            assert_eq!(values[1].0, "key1");
            assert_eq!(values[1].1, "value1");
        }
    }

    // test memory delete
    #[tokio::test]
    async fn test_memory_delete() {
        let mut memory = Memory::new();
        memory
            .set("key".to_string(), "value".to_string(), 0)
            .await
            .unwrap();
        assert_eq!(memory.get("key".to_string()).await.unwrap(), "value");
        memory.delete("key".to_string()).await.unwrap();
        assert_eq!(memory.get("key".to_string()).await.is_err(), true);
    }
}
