use super::Context;
use leaf_common::errors::Error;
use leaf_host_impl::http::{HttpHandler, Request as LeafRequest, Response as LeafResponse};
use leaf_host_kv::Memory;
use tokio::time::Instant;
use tracing::info;
use wasmtime::{
    component::{Component, Instance, InstancePre, Linker},
    Config, Engine, Store,
};

pub struct Worker {
    path: String,
    engine: Engine,
    component: Component,
    enable_wasi: bool,

    /// If wasi enable, use instance_pre to cache worker
    instance_pre: Option<InstancePre<Context>>,

    /// If wasi disable, use instance to cache worker
    instance: Option<Instance>,
    store: Option<Store<Context>>,
    exports: Option<HttpHandler>,

    /// Whether the worker is trapped.If the worker is trapped, it needs re-create.
    is_trapped: bool,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Worker")
            .field("path", &self.path)
            .field("enable_wasi", &self.enable_wasi)
            .finish()
    }
}

impl Worker {
    pub async fn new(path: &str, enable_wasi: bool) -> Result<Self, Error> {
        let start_time = Instant::now();
        let config = create_wasmtime_config();
        let engine = Engine::new(&config).map_err(Error::InitEngine)?;
        let component = Component::from_file(&engine, path)
            .map_err(|e| Error::ReadWasmComponent(e, String::from(path)))?;

        let mut worker = Self {
            path: String::from(path),
            engine,
            component,
            instance: None,
            store: None,
            exports: None,
            instance_pre: None,
            is_trapped: false,
            enable_wasi,
        };
        if worker.enable_wasi {
            worker.create_instance_pre()?;
        } else {
            worker.create_instance().await?;
        }

        info!(
            "[Worker] new worker, path: {}, took: {:?} ms",
            path,
            start_time.elapsed().as_millis()
        );

        Ok(worker)
    }

    /// If the worker is trapped, it needs re-create instance.
    async fn create_instance(&mut self) -> Result<(), Error> {
        let start_time = Instant::now();
        let ctx = Context::new(0, Box::new(Memory::new()));
        let mut store = Store::new(&self.engine, ctx);
        let mut linker: Linker<Context> = Linker::new(&self.engine);
        leaf_host_impl::http::add_to_linker(&mut linker, Context::fetch).map_err(|e| {
            Error::InstantiateWasmComponent(e, "leaf_host_impl::http::add_to_linker".to_string())
        })?;
        leaf_host_impl::kv::add_to_linker(&mut linker, Context::kv).map_err(|e| {
            Error::InstantiateWasmComponent(e, "leaf_host_impl::kv::add_to_linker".to_string())
        })?;
        if self.enable_wasi {
            wasi_host::add_to_linker(&mut linker, Context::wasi).map_err(|e| {
                Error::InstantiateWasmComponent(e, "wasi_host::add_to_linker".to_string())
            })?;
        }
        let (exports, instance) =
            HttpHandler::instantiate_async(&mut store, &self.component, &linker)
                .await
                .map_err(|e| {
                    Error::InstantiateWasmComponent(e, "HttpHandler::instantiate_async".to_string())
                })?;
        self.instance = Some(instance);
        self.store = Some(store);
        self.exports = Some(exports);
        self.is_trapped = false;
        info!(
            "[Worker] new instance, path: {}, took: {:?} ms",
            self.path.clone(),
            start_time.elapsed().as_millis()
        );
        Ok(())
    }

    /// If this worker enable wasi, use instance_pre to initialize the worker.
    fn create_instance_pre(&mut self) -> Result<(), Error> {
        let start_time = Instant::now();

        // create linker
        let mut linker: Linker<Context> = Linker::new(&self.engine);
        leaf_host_impl::http::add_to_linker(&mut linker, Context::fetch).map_err(|e| {
            Error::InstantiateWasmComponent(e, "leaf_host_impl::http::add_to_linker".to_string())
        })?;
        leaf_host_impl::kv::add_to_linker(&mut linker, Context::kv).map_err(|e| {
            Error::InstantiateWasmComponent(e, "leaf_host_impl::kv::add_to_linker".to_string())
        })?;
        wasi_host::add_to_linker(&mut linker, Context::wasi).map_err(|e| {
            Error::InstantiateWasmComponent(e, "wasi_host::add_to_linker".to_string())
        })?;

        // create instance_pre
        let instance_pre = linker.instantiate_pre(&self.component).map_err(|e| {
            Error::InstantiateWasmComponent(e, "linker.instantiate_pre".to_string())
        })?;
        self.instance_pre = Some(instance_pre);
        self.is_trapped = false;
        info!(
            "[Worker] new instance_pre, path: {}, took: {:?} ms",
            self.path.clone(),
            start_time.elapsed().as_millis()
        );
        Ok(())
    }

    async fn handle_request_with_instance(
        &mut self,
        req: LeafRequest<'_>,
    ) -> Result<LeafResponse, Error> {
        let mut store = self.store.as_mut().unwrap();
        store.data_mut().fetch().req_id = req.request_id;
        let exports = self.exports.as_ref().unwrap();
        let res = exports
            .http_handler()
            .call_handle_request(&mut store, req)
            .await
            .map_err(Error::InvokeComponentExportFunction)?;
        Ok(res)
    }

    async fn handle_request_with_instance_pre(
        &mut self,
        req: LeafRequest<'_>,
    ) -> Result<LeafResponse, Error> {
        let context = Context::new(req.request_id, Box::new(Memory::new()));
        let mut store = Store::new(&self.engine, context);
        let instance = self
            .instance_pre
            .as_ref()
            .unwrap()
            .instantiate_async(&mut store)
            .await
            .map_err(|e| {
                Error::InstantiateWasmComponent(e, "instance_pre::instantiate_async".to_string())
            })?;
        let exports = HttpHandler::new(&mut store, &instance).map_err(|e| {
            Error::InstantiateWasmComponent(
                e,
                "instance_pre::HttpHandler::instantiate_async".to_string(),
            )
        })?;
        let res = exports
            .http_handler()
            .call_handle_request(&mut store, req)
            .await
            .map_err(Error::InvokeComponentExportFunction)?;
        Ok(res)
    }

    pub async fn handle_request(&mut self, req: LeafRequest<'_>) -> Result<LeafResponse, Error> {
        if self.enable_wasi {
            return self.handle_request_with_instance_pre(req).await;
        }
        self.handle_request_with_instance(req).await
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
    use leaf_host_impl::http::Request;

    let sample_wasm_file = "../../tests/sample.wasm";

    let mut worker = Worker::new(sample_wasm_file, false).await.unwrap();

    for _ in 1..10 {
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

#[tokio::test]
async fn run_wasi_worker_test() {
    use super::Worker;
    use leaf_host_impl::http::Request;

    // TODO: use real wasi wasm file
    let sample_wasm_file = "../../tests/sample.wasm";

    let mut worker = Worker::new(sample_wasm_file, true).await.unwrap();

    for _ in 1..10 {
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
