use super::Context;
use crate::common::errors::Error;
use crate::wit::fetch;
use log::info;
use tokio::time::Instant;
use wasmtime::{
    component::{Component, InstancePre, Linker},
    Config, Engine,
};

pub struct PreWorker {
    pub engine: Engine,
    pub component: Component,
    pub path: String,
    pub instance_pre: InstancePre<Context>,
}

impl std::fmt::Debug for PreWorker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("PreWorker")
            .field("path", &self.path)
            .finish()
    }
}

impl PreWorker {
    pub fn new(path: &str) -> Result<Self, Error> {
        let start_time = Instant::now();
        let config = create_wasmtime_config();
        let engine = Engine::new(&config).map_err(Error::InitEngine)?;
        let component = Component::from_file(&engine, path)
            .map_err(|e| Error::ReadWasmComponent(e, String::from(path)))?;

        // create linker
        let mut linker: Linker<Context> = Linker::new(&engine);
        fetch::add_to_linker(&mut linker, Context::fetch)
            .map_err(Error::InstantiateWasmComponent)?;
        wasi_host::add_to_linker(&mut linker, Context::wasi)
            .map_err(Error::InstantiateWasmComponent)?;

        // create instance_pre
        let instance_pre = linker
            .instantiate_pre(&component)
            .map_err(Error::InstantiateWasmComponent)?;

        info!(
            "[PreWorker] new instance, path: {}, took: {:?} ms",
            path,
            start_time.elapsed().as_millis()
        );

        Ok(Self {
            engine,
            component,
            path: String::from(path),
            instance_pre,
        })
    }
}

pub fn create_wasmtime_config() -> Config {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    config
}

#[tokio::test]
async fn run_wasm_pre_worker_test() {
    use super::Context;
    use super::PreWorker;
    use crate::wit::{LeafHttp, Request};
    use wasmtime::Store;

    let sample_wasm_file = "./tests/sample.wasm";

    let pre_worker = PreWorker::new(sample_wasm_file).unwrap();
    let mut store = Store::new(&pre_worker.engine, Context::new());
    let instance = pre_worker
        .instance_pre
        .instantiate_async(&mut store)
        .await
        .unwrap();
    let exports = LeafHttp::new(&mut store, &instance).unwrap();

    for _ in 1..10 {
        let headers: Vec<(&str, &str)> = vec![];
        let req = Request {
            method: "GET",
            uri: "/abc",
            headers: &headers,
            body: Some("xxxyyy".as_bytes()),
        };

        let resp = exports.handle_request(&mut store, req).await.unwrap();
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
