use super::Worker;
use async_trait::async_trait;
use deadpool::managed;

#[derive(Debug)]
pub struct Manager {
    path: String,
}

impl Manager {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Worker;
    type Error = crate::errors::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(Worker::new(&self.path)?)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;
