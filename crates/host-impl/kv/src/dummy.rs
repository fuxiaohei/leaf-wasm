use super::{Error, Provider};
pub struct Dummy {}

impl Dummy {
    pub fn new() -> Self {
        Dummy {}
    }
}

#[async_trait::async_trait]
impl Provider for Dummy {
    async fn get(&mut self, _key: String) -> Result<String, Error> {
        Ok("dummy".to_string())
    }
    async fn set(&mut self, _key: String, _value: String, _ttl: u32) -> Result<(), Error> {
        Ok(())
    }
    async fn delete(&mut self, _key: String) -> Result<(), Error> {
        Ok(())
    }
    async fn list(&mut self, _prefix: String) -> Result<Vec<(String, String)>, Error> {
        Ok(vec![])
    }
}
