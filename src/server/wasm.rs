use crate::errors::Error;
use tokio::sync::{Mutex, MutexGuard, Notify};
use wasmtime::{
    component::{Component, Instance, Linker},
    Config, Engine, Store,
};

pub struct ComponentContext {
    pub engine: Engine,
    pub component: Component,
    instance: Instance,
    store: Store<()>,
    exports: super::LeafHttp,
}

impl ComponentContext {
    pub fn new(path: &str) -> Result<Self, Error> {
        let config = create_wasmtime_config();
        let engine = Engine::new(&config).map_err(Error::InitEngine)?;
        let component = Component::from_file(&engine, path)
            .map_err(|e| Error::ReadWasmComponent(e, String::from(path)))?;

        let mut store = Store::new(&engine, ());
        let linker: Linker<()> = Linker::new(&engine);
        let (exports, instance) = super::LeafHttp::instantiate(&mut store, &component, &linker)
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
    config
}

pub struct ComponentPool {
    values: Mutex<ComponentContext>,
    notify: Notify,
}

impl ComponentPool {
    pub fn new(path: &str) -> Result<Self, Error> {
        let values = ComponentContext::new(path)?;
        Ok(Self {
            values: Mutex::new(values),
            notify: Notify::new(),
        })
    }

    pub async fn get(&self) -> Result<MutexGuard<ComponentContext>, Error> {
        let value = self.values.lock().await;
        Ok(value)
    }

    pub fn put(&self) {
        self.notify.notify_one();
    }
}
