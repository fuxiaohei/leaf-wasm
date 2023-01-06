use super::PreWorker;
use async_trait::async_trait;
use deadpool::managed;

#[derive(Debug)]
pub struct PreManager {
    path: String,
}

impl PreManager {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl managed::Manager for PreManager {
    type Type = PreWorker;
    type Error = crate::common::errors::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        Ok(PreWorker::new(&self.path)?)
    }

    async fn recycle(&self, _obj: &mut Self::Type) -> managed::RecycleResult<Self::Error> {
        Ok(())
    }
}

pub type PrePool = managed::Pool<PreManager>;

#[tokio::test]
async fn run_worker_pre_pool_test() {
    use super::{Context, PreManager, PrePool};
    use crate::wit::{LeafHttp, Request};
    use wasmtime::Store;

    let sample_wasm_file = "./tests/sample.wasm";
    let mgr = PreManager::new(sample_wasm_file.to_string());
    let pool = PrePool::builder(mgr).build().unwrap();

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
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let mut store = Store::new(&worker.engine, Context::new());
        let instance = worker
            .instance_pre
            .instantiate_async(&mut store)
            .await
            .unwrap();
        let exports = LeafHttp::new(&mut store, &instance).unwrap();

        let resp = exports.handle_request(&mut store, req).await.unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("xxxyyy".as_bytes().to_vec()));
    }

    let status = pool.status();
    assert_eq!(status.size, 1);
    assert_eq!(status.available, 1);
}
