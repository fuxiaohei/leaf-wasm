use crate::errors::Error;
use crate::wit::LeafHttp;
use wasmtime::{
    component::{Component, Instance, Linker},
    Config, Engine, Store,
};

pub struct Worker {
    pub engine: Engine,
    pub component: Component,
    pub instance: Instance,
    pub store: Store<()>,
    pub exports: LeafHttp,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Worker").finish()
    }
}

impl Worker {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config = create_wasmtime_config();
        let engine = Engine::new(&config).map_err(Error::InitEngine)?;
        let component = Component::from_file(&engine, path)
            .map_err(|e| Error::ReadWasmComponent(e, String::from(path)))?;

        let mut store = Store::new(&engine, ());
        let linker: Linker<()> = Linker::new(&engine);
        let (exports, instance) =
            LeafHttp::instantiate_async(&mut store, &component, &linker)
                .await
                .map_err(Error::InstantiateWasmComponent)?;

        Ok(Self {
            engine,
            component,
            instance,
            store,
            exports,
        })
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
