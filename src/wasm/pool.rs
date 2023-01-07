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
    type Error = crate::common::errors::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(Worker::new(&self.path, false).await?)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;

#[tokio::test]
async fn run_worker_pool_test() {
    use super::{Manager, Pool};
    use crate::wit::Request;

    let sample_wasm_file = "./tests/sample.wasm";
    let mgr = Manager::new(sample_wasm_file.to_string());
    let pool = Pool::builder(mgr).build().unwrap();

    let status = pool.status();
    assert_eq!(status.size, 0);
    assert_eq!(status.available, 0);

    {
        let mut worker = pool.get().await.unwrap();
        let worker = worker.as_mut();

        let status = pool.status();
        assert_eq!(status.size, 1);
        assert_eq!(status.available, 0);

        let headers: Vec<(&str, &str)> = vec![];
        let req = Request {
            id: 1,
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = worker.handle_request(req).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("xxxyyy".as_bytes().to_vec()));
    }

    let status = pool.status();
    assert_eq!(status.size, 1);
    assert_eq!(status.available, 1);
}
