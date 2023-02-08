use super::Worker;
use async_trait::async_trait;
use deadpool::managed;
use leaf_host_kv::Memory as MemoryKv;

#[derive(Debug)]
pub struct Manager {
    path: String,
    enable_wasi: bool,
}

impl Manager {
    pub fn new(path: String, enable_wasi: bool) -> Self {
        Self { path, enable_wasi }
    }
}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Worker;
    type Error = leaf_common::errors::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(Worker::new(&self.path, self.enable_wasi, MemoryKv::new()).await?)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type Pool = managed::Pool<Manager>;

#[tokio::test]
async fn run_worker_pool_test() {
    use super::{Manager, Pool};
    use leaf_host_impl::http::Request;

    let sample_wasm_file = "../../tests/sample.wasm";
    let mgr = Manager::new(sample_wasm_file.to_string(), false);
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
            request_id: 1,
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = worker.handle_request(req).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("Hello, World".as_bytes().to_vec()));
    }

    let status = pool.status();
    assert_eq!(status.size, 1);
    assert_eq!(status.available, 1);
}
