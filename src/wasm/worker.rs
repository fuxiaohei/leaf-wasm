use crate::errors::Error;
use crate::wit::{fetch, LeafHttp};
use log::info;
use tokio::time::Instant;
use wasmtime::{
    component::{Component, Instance, Linker},
    Config, Engine, Store,
};

pub struct Worker {
    pub engine: Engine,
    pub component: Component,

    pub instance: Instance,
    pub store: Store<fetch::FetchImpl>,
    pub exports: LeafHttp,

    pub is_trapped: bool,
    pub path: String,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Worker").finish()
    }
}

impl Worker {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let start_time = Instant::now();
        let config = create_wasmtime_config();
        let engine = Engine::new(&config).map_err(Error::InitEngine)?;
        let component = Component::from_file(&engine, path)
            .map_err(|e| Error::ReadWasmComponent(e, String::from(path)))?;

        // add the fetch implementation to the store
        let fetch_impl = fetch::FetchImpl { req_id: 0 };
        let mut store = Store::new(&engine, fetch_impl);

        // add the fetch implementation to the linker
        let mut linker: Linker<fetch::FetchImpl> = Linker::new(&engine);
        fetch::add_to_linker(&mut linker, |f: &mut fetch::FetchImpl| f)
            .map_err(Error::InstantiateWasmComponent)?;

        let (exports, instance) = LeafHttp::instantiate_async(&mut store, &component, &linker)
            .await
            .map_err(Error::InstantiateWasmComponent)?;

        info!(
            "[Worker] new instance, path: {}, took: {:?} ms",
            path,
            start_time.elapsed().as_millis()
        );

        Ok(Self {
            engine,
            component,
            instance,
            store,
            exports,
            is_trapped: false,
            path: String::from(path),
        })
    }

    /// Renew the instance of the worker. 
    /// If the worker is trapped, it can't be re-used, show 'cannot reenter component instance'. 
    /// We need to create a new instance.
    pub async fn renew(&mut self) -> Result<(), Error> {
        let start_time = Instant::now();

        let fetch_impl = fetch::FetchImpl { req_id: 0 };
        let mut store = Store::new(&self.engine, fetch_impl);
        let mut linker: Linker<fetch::FetchImpl> = Linker::new(&self.engine);
        fetch::add_to_linker(&mut linker, |f: &mut fetch::FetchImpl| f)
            .map_err(Error::InstantiateWasmComponent)?;
        let (exports, instance) = LeafHttp::instantiate_async(&mut store, &self.component, &linker)
            .await
            .map_err(Error::InstantiateWasmComponent)?;
        self.exports = exports;
        self.instance = instance;
        self.store = store;

        info!(
            "[Worker] renew instance, path: {}, took: {:?} ms",
            self.path,
            start_time.elapsed().as_millis()
        );

        Ok(())
    }
}

fn create_wasmtime_config() -> Config {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config
}

#[tokio::test]
async fn run_wasm_worker_test() {
    use super::Worker;
    use crate::wit::Request;

    let sample_wasm_file = "./etc/sample.wasm";

    let mut worker = Worker::new(sample_wasm_file).await.unwrap();

    for _ in 1..10 {
        let headers: Vec<(&str, &str)> = vec![];
        let req = Request {
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = worker
            .exports
            .handle_request(&mut worker.store, req)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, Some("xxxyyy".as_bytes().to_vec()));

        let headers = resp.headers;
        for (key, value) in headers {
            if key == "X-Request-Method" {
                assert_eq!(value, "GET");
            }
            if key == "X-Request-Url" {
                assert_eq!(value, "/abc");
            }
        }
    }
}
