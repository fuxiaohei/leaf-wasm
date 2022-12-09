use crate::errors::Error;
use wasmtime::{
    component::{Component, Instance, Linker},
    Config, Engine, Store,
};

pub struct Worker {
    pub engine: Engine,
    pub component: Component,
    pub instance: Instance,
    pub store: Store<()>,
    pub exports: super::LeafHttp,
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
            super::LeafHttp::instantiate_async(&mut store, &component, &linker)
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
